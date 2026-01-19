use std::collections::HashSet;
#[cfg(target_os = "linux")]
use std::ffi::CString;
#[cfg(target_os = "linux")]
use std::os::raw::{c_char, c_int};
use std::sync::atomic::{AtomicBool, AtomicU16, AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

#[cfg(target_os = "linux")]
use alsa::card::Card;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, FromSample, Sample, SampleFormat, SizedSample, Stream, StreamConfig};

use crate::settings::AudioSettings;
use crate::overlay;

const MAX_RECORDING_SECONDS: u32 = 600;

#[derive(Clone, serde::Serialize)]
pub struct AudioDevice {
  pub id: String,
  pub name: String,
  pub is_default: bool,
}

pub enum AudioCommand {
  Start(AudioSettings, i64, mpsc::Sender<Result<(), String>>),
  Snapshot(usize, mpsc::Sender<Result<AudioSnapshot, String>>),
  Stop(mpsc::Sender<Result<RecordedAudio, String>>),
}

pub fn start_worker() -> mpsc::Sender<AudioCommand> {
  let (tx, rx) = mpsc::channel();
  thread::spawn(move || {
    let mut recorder: Option<Recorder> = None;
    for command in rx {
      match command {
        AudioCommand::Start(settings, started_at_ms, reply) => {
          if recorder.is_some() {
            let _ = reply.send(Err("Recorder already running".to_string()));
            continue;
          }
          match Recorder::start(&settings, started_at_ms) {
            Ok(active) => {
              recorder = Some(active);
              let _ = reply.send(Ok(()));
            }
            Err(err) => {
              let _ = reply.send(Err(err));
            }
          }
        }
        AudioCommand::Stop(reply) => {
          match recorder.take() {
            Some(active) => {
              let result = active.stop();
              let _ = reply.send(result);
            }
            None => {
              let _ = reply.send(Err("No active recorder found".to_string()));
            }
          }
        }
        AudioCommand::Snapshot(from_index, reply) => {
          match recorder.as_ref() {
            Some(active) => {
              let result = active.snapshot(from_index);
              let _ = reply.send(result);
            }
            None => {
              let _ = reply.send(Err("No active recorder found".to_string()));
            }
          }
        }
      }
    }
  });
  tx
}

pub fn start_recording(
  tx: &mpsc::Sender<AudioCommand>,
  settings: AudioSettings,
  started_at_ms: i64,
) -> Result<(), String> {
  let (reply_tx, reply_rx) = mpsc::channel();
  tx.send(AudioCommand::Start(settings, started_at_ms, reply_tx))
    .map_err(|_| "Audio worker unavailable".to_string())?;
  reply_rx
    .recv()
    .map_err(|_| "Audio worker unavailable".to_string())?
}

pub fn stop_recording(tx: &mpsc::Sender<AudioCommand>) -> Result<RecordedAudio, String> {
  let (reply_tx, reply_rx) = mpsc::channel();
  tx.send(AudioCommand::Stop(reply_tx))
    .map_err(|_| "Audio worker unavailable".to_string())?;
  reply_rx
    .recv()
    .map_err(|_| "Audio worker unavailable".to_string())?
}

pub fn snapshot_audio(
  tx: &mpsc::Sender<AudioCommand>,
  from_index: usize,
) -> Result<AudioSnapshot, String> {
  let (reply_tx, reply_rx) = mpsc::channel();
  tx.send(AudioCommand::Snapshot(from_index, reply_tx))
    .map_err(|_| "Audio worker unavailable".to_string())?;
  reply_rx
    .recv()
    .map_err(|_| "Audio worker unavailable".to_string())?
}

pub fn list_input_devices() -> Vec<AudioDevice> {
  silence_alsa_errors();
  let host = cpal::default_host();
  let mut devices = Vec::new();
  let mut seen = HashSet::new();
  let mut discovered = Vec::new();

  // Add default option
  devices.push(AudioDevice {
    id: "default".to_string(),
    name: "Default".to_string(),
    is_default: true,
  });

  // Get default device name for comparison
  let default_name = host.default_input_device().and_then(|d| d.name().ok());

  let raw_names = match host.input_devices() {
    Ok(input_devices) => input_devices.filter_map(|device| device.name().ok()).collect(),
    Err(_) => Vec::new(),
  };

  let plughw_keys: HashSet<String> = raw_names
    .iter()
    .filter_map(|name| name.strip_prefix("plughw:").map(|rest| format!("hw:{rest}")))
    .collect();

  for name in raw_names {
    if name.eq_ignore_ascii_case("default") {
      continue;
    }
    if !should_include_device_name(&name) {
      continue;
    }
    if name.starts_with("hw:") && plughw_keys.contains(&name) {
      continue;
    }
    if !seen.insert(name.clone()) {
      continue;
    }

    let is_default = default_name.as_ref().map(|d| d == &name).unwrap_or(false);
    let label = format_device_label(&name, is_default);
    discovered.push(AudioDevice {
      id: name.clone(),
      name: label,
      is_default,
    });
  }

  discovered.sort_by(|a, b| a.name.cmp(&b.name));
  devices.extend(discovered);
  devices
}

fn should_include_device_name(name: &str) -> bool {
  let lower = name.to_lowercase();
  let blocked_prefixes = [
    "pipewire",
    "pulse",
    "sysdefault",
    "front",
    "surround",
    "iec958",
    "spdif",
    "hdmi",
    "dmix",
    "dsnoop",
    "null",
  ];

  !blocked_prefixes.iter().any(|prefix| lower.starts_with(prefix))
}

fn format_device_label(name: &str, is_default: bool) -> String {
  #[cfg(target_os = "linux")]
  let base = alsa_friendly_name(name).unwrap_or_else(|| name.to_string());
  #[cfg(not(target_os = "linux"))]
  let base = name.to_string();

  if is_default {
    format!("{base} (System Default)")
  } else {
    base
  }
}

#[cfg(target_os = "linux")]
fn alsa_friendly_name(name: &str) -> Option<String> {
  let (card_id, dev) = parse_alsa_device_name(name)?;
  let longname = alsa_card_longname(&card_id).unwrap_or(card_id);
  let label = match dev.as_deref() {
    Some("0") | None => longname,
    Some(dev_id) => format!("{longname} (Device {dev_id})"),
  };
  Some(label)
}

#[cfg(target_os = "linux")]
fn parse_alsa_device_name(name: &str) -> Option<(String, Option<String>)> {
  let rest = name.strip_prefix("plughw:").or_else(|| name.strip_prefix("hw:"))?;
  let mut card: Option<String> = None;
  let mut dev: Option<String> = None;

  for part in rest.split(',') {
    if let Some(value) = part.strip_prefix("CARD=") {
      card = Some(value.to_string());
      continue;
    }
    if let Some(value) = part.strip_prefix("DEV=") {
      dev = Some(value.to_string());
      continue;
    }
    if card.is_none() {
      card = Some(part.to_string());
      continue;
    }
    if dev.is_none() {
      dev = Some(part.to_string());
    }
  }

  card.map(|card_id| (card_id, dev))
}

#[cfg(target_os = "linux")]
fn alsa_card_longname(card_id: &str) -> Option<String> {
  let cstr = CString::new(card_id).ok()?;
  let card = Card::from_str(&cstr).ok()?;
  let longname = card.get_longname().ok()?;
  let trimmed = longname
    .split_once(" at ")
    .map(|(head, _)| head.trim())
    .unwrap_or(longname.trim());
  Some(trimmed.to_string())
}

fn silence_alsa_errors() {
  #[cfg(target_os = "linux")]
  unsafe {
    extern "C" fn alsa_error_handler(
      _file: *const c_char,
      _line: c_int,
      _func: *const c_char,
      _err: c_int,
      _fmt: *const c_char,
      _arg: *mut alsa_sys::__va_list_tag,
    ) {
    }

    let _ = alsa_sys::snd_lib_error_set_local(Some(alsa_error_handler));
  }
}

pub struct Recorder {
  stream: Stream,
  samples: Arc<Mutex<Vec<f32>>>,
  total_samples: Arc<AtomicUsize>,
  sample_rate: u32,
  channels: u16,
  meter_stop: Arc<AtomicBool>,
  meter_thread: Option<thread::JoinHandle<()>>,
}

#[derive(Clone)]
pub struct RecordedAudio {
  pub samples: Vec<f32>,
  pub sample_rate: u32,
  pub channels: u16,
}

pub struct AudioSnapshot {
  pub samples: Vec<f32>,
  pub sample_rate: u32,
  pub channels: u16,
  pub total_samples: usize,
}

impl Recorder {
  pub fn start(settings: &AudioSettings, started_at_ms: i64) -> Result<Self, String> {
    silence_alsa_errors();
    let host = cpal::default_host();
    let device = select_device(&host, &settings.input_device_id)?;
    let (config, sample_format) = select_config(&device, settings)?;

    let samples = Arc::new(Mutex::new(Vec::new()));
    let gain = db_to_gain(settings.input_gain_db);
    let level = Arc::new(AtomicU16::new(0));
    let meter_stop = Arc::new(AtomicBool::new(false));
    let total_samples = Arc::new(AtomicUsize::new(0));

    let gate_enabled = settings.noise_gate_enabled;
    let gate_threshold = settings.noise_gate_threshold.clamp(0.0, 1.0);
    let vad_enabled = settings.vad_enabled;
    let vad_threshold = settings.vad_threshold.clamp(0.0, 1.0);
    let vad_silence_ms = settings.vad_silence_ms;
    let vad_resume_ms = settings.vad_resume_ms;

    let stream = match sample_format {
      SampleFormat::F32 => build_stream::<f32>(&device, &config, samples.clone(), total_samples.clone(), level.clone(), gain, gate_enabled, gate_threshold, vad_enabled, vad_threshold, vad_silence_ms, vad_resume_ms)?,
      SampleFormat::I16 => build_stream::<i16>(&device, &config, samples.clone(), total_samples.clone(), level.clone(), gain, gate_enabled, gate_threshold, vad_enabled, vad_threshold, vad_silence_ms, vad_resume_ms)?,
      SampleFormat::U16 => build_stream::<u16>(&device, &config, samples.clone(), total_samples.clone(), level.clone(), gain, gate_enabled, gate_threshold, vad_enabled, vad_threshold, vad_silence_ms, vad_resume_ms)?,
      SampleFormat::I8 => build_stream::<i8>(&device, &config, samples.clone(), total_samples.clone(), level.clone(), gain, gate_enabled, gate_threshold, vad_enabled, vad_threshold, vad_silence_ms, vad_resume_ms)?,
      SampleFormat::U8 => build_stream::<u8>(&device, &config, samples.clone(), total_samples.clone(), level.clone(), gain, gate_enabled, gate_threshold, vad_enabled, vad_threshold, vad_silence_ms, vad_resume_ms)?,
      SampleFormat::I32 => build_stream::<i32>(&device, &config, samples.clone(), total_samples.clone(), level.clone(), gain, gate_enabled, gate_threshold, vad_enabled, vad_threshold, vad_silence_ms, vad_resume_ms)?,
      SampleFormat::U32 => build_stream::<u32>(&device, &config, samples.clone(), total_samples.clone(), level.clone(), gain, gate_enabled, gate_threshold, vad_enabled, vad_threshold, vad_silence_ms, vad_resume_ms)?,
      SampleFormat::I64 => build_stream::<i64>(&device, &config, samples.clone(), total_samples.clone(), level.clone(), gain, gate_enabled, gate_threshold, vad_enabled, vad_threshold, vad_silence_ms, vad_resume_ms)?,
      SampleFormat::U64 => build_stream::<u64>(&device, &config, samples.clone(), total_samples.clone(), level.clone(), gain, gate_enabled, gate_threshold, vad_enabled, vad_threshold, vad_silence_ms, vad_resume_ms)?,
      SampleFormat::F64 => build_stream::<f64>(&device, &config, samples.clone(), total_samples.clone(), level.clone(), gain, gate_enabled, gate_threshold, vad_enabled, vad_threshold, vad_silence_ms, vad_resume_ms)?,
      _ => return Err("Unsupported audio sample format".to_string()),
    };

    stream.play().map_err(|err| err.to_string())?;

    let meter_level = level.clone();
    let meter_stop_flag = meter_stop.clone();
    let meter_thread = thread::spawn(move || {
      while !meter_stop_flag.load(Ordering::Relaxed) {
        let raw = meter_level.load(Ordering::Relaxed) as f32;
        let normalized = (raw / 1000.0).clamp(0.0, 1.0);
        let _ = overlay::write_state(true, Some(started_at_ms), Some(normalized));
        thread::sleep(Duration::from_millis(120));
      }
    });

    Ok(Self {
      stream,
      samples,
      total_samples,
      sample_rate: config.sample_rate.0,
      channels: config.channels,
      meter_stop,
      meter_thread: Some(meter_thread),
    })
  }

  pub fn stop(mut self) -> Result<RecordedAudio, String> {
    self.meter_stop.store(true, Ordering::Relaxed);
    if let Some(handle) = self.meter_thread.take() {
      let _ = handle.join();
    }
    let _ = self.stream.pause();
    let samples = self
      .samples
      .lock()
      .map_err(|_| "audio buffer lock poisoned".to_string())
      .map(|mut guard| std::mem::take(&mut *guard))?;

    Ok(RecordedAudio {
      samples,
      sample_rate: self.sample_rate,
      channels: self.channels,
    })
  }

  pub fn snapshot(&self, from_index: usize) -> Result<AudioSnapshot, String> {
    let guard = self
      .samples
      .lock()
      .map_err(|_| "audio buffer lock poisoned".to_string())?;
    let total_samples = self.total_samples.load(Ordering::Relaxed);
    let base = total_samples.saturating_sub(guard.len());
    let start = if from_index <= base {
      0
    } else {
      (from_index - base).min(guard.len())
    };
    let samples = guard[start..].to_vec();
    Ok(AudioSnapshot {
      samples,
      sample_rate: self.sample_rate,
      channels: self.channels,
      total_samples,
    })
  }
}

fn select_device(host: &cpal::Host, input_device_id: &str) -> Result<cpal::Device, String> {
  if input_device_id != "default" {
    if let Ok(mut devices) = host.input_devices() {
      if let Some(device) = devices.find(|device| {
        device
          .name()
          .map(|name| name == input_device_id)
          .unwrap_or(false)
      }) {
        return Ok(device);
      }
    }
  }

  host
    .default_input_device()
    .ok_or_else(|| "No input audio device available".to_string())
}

fn select_config(
  device: &cpal::Device,
  settings: &AudioSettings,
) -> Result<(StreamConfig, SampleFormat), String> {
  let mut fallback: Option<(StreamConfig, SampleFormat)> = None;
  let target_rate = settings.sample_rate_hz;
  let target_channels = settings.channels;

  let configs = device
    .supported_input_configs()
    .map_err(|err| err.to_string())?;

  for config_range in configs {
    let min_rate = config_range.min_sample_rate().0;
    let max_rate = config_range.max_sample_rate().0;
    let sample_rate = clamp_sample_rate(target_rate, min_rate, max_rate);
    let config = config_range.with_sample_rate(cpal::SampleRate(sample_rate));
    let sample_format = config.sample_format();

    if fallback.is_none() {
      fallback = Some((to_stream_config(&config), sample_format));
    }

    if config.channels() == target_channels {
      return Ok((to_stream_config(&config), sample_format));
    }
  }

  if let Some((config, sample_format)) = fallback {
    return Ok((config, sample_format));
  }

  let config = device
    .default_input_config()
    .map_err(|err| err.to_string())?;
  let sample_format = config.sample_format();
  Ok((config.into(), sample_format))
}

fn to_stream_config(config: &cpal::SupportedStreamConfig) -> StreamConfig {
  StreamConfig {
    channels: config.channels(),
    sample_rate: config.sample_rate(),
    buffer_size: BufferSize::Default,
  }
}

fn clamp_sample_rate(target: u32, min: u32, max: u32) -> u32 {
  if target < min {
    min
  } else if target > max {
    max
  } else {
    target
  }
}

fn db_to_gain(db: f32) -> f32 {
  if db == 0.0 {
    return 1.0;
  }
  10.0_f32.powf(db / 20.0)
}

fn build_stream<T>(
  device: &cpal::Device,
  config: &StreamConfig,
  samples: Arc<Mutex<Vec<f32>>>,
  total_samples: Arc<AtomicUsize>,
  level: Arc<AtomicU16>,
  gain: f32,
  gate_enabled: bool,
  gate_threshold: f32,
  vad_enabled: bool,
  vad_threshold: f32,
  vad_silence_ms: u32,
  vad_resume_ms: u32,
) -> Result<Stream, String>
where
  T: SizedSample + Send + 'static,
  f32: FromSample<T>,
{
  let err_fn = |err| {
    eprintln!("Audio input stream error: {err}");
  };

  let vad_state = Arc::new(Mutex::new(VadState {
    active: !vad_enabled,
    silence_ms: 0,
    speech_ms: 0,
  }));
  let sample_rate = config.sample_rate.0.max(1);
  let channels = config.channels.max(1) as usize;
  let max_samples = (sample_rate as usize)
    .saturating_mul(channels)
    .saturating_mul(MAX_RECORDING_SECONDS as usize);

  device
    .build_input_stream(
      config,
      move |data: &[T], _| {
        if data.is_empty() {
          return;
        }

        let mut sum = 0.0_f32;
        for sample in data {
          let value = f32::from_sample(*sample) * gain;
          sum += value * value;
        }

        let rms = (sum / data.len() as f32).sqrt();
        let normalized = (rms * 2.5).clamp(0.0, 1.0);
        level.store((normalized * 1000.0) as u16, Ordering::Relaxed);

        if vad_enabled {
          let frames = data.len() / channels;
          let chunk_ms = if sample_rate > 0 {
            ((frames as u64).saturating_mul(1000) / sample_rate as u64) as u32
          } else {
            0
          };
          let speech = rms >= vad_threshold;

          if let Ok(mut state) = vad_state.lock() {
            if state.active {
              if speech {
                state.silence_ms = 0;
              } else {
                state.silence_ms = state.silence_ms.saturating_add(chunk_ms);
                if state.silence_ms >= vad_silence_ms {
                  state.active = false;
                  state.speech_ms = 0;
                }
              }
            } else if speech {
              state.speech_ms = state.speech_ms.saturating_add(chunk_ms);
              if state.speech_ms >= vad_resume_ms {
                state.active = true;
                state.silence_ms = 0;
              }
            } else {
              state.speech_ms = 0;
            }

            if !state.active {
              return;
            }
          }
        }

        if gate_enabled && rms < gate_threshold {
          return;
        }

        let mut buffer = match samples.lock() {
          Ok(buffer) => buffer,
          Err(_) => return,
        };

        let mut start_index = 0;
        if max_samples > 0 {
          let incoming = data.len();
          let current_len = buffer.len();
          let total = current_len.saturating_add(incoming);
          if total > max_samples {
            let overflow = total - max_samples;
            if overflow >= current_len {
              buffer.clear();
              start_index = overflow.saturating_sub(current_len).min(incoming);
            } else {
              buffer.drain(0..overflow);
            }
          }
        }

        let slice = &data[start_index..];
        if slice.is_empty() {
          return;
        }
        total_samples.fetch_add(slice.len(), Ordering::Relaxed);
        buffer.reserve(slice.len());
        for sample in slice {
          buffer.push(f32::from_sample(*sample) * gain);
        }
      },
      err_fn,
      None,
    )
    .map_err(|err| err.to_string())
}

struct VadState {
  active: bool,
  silence_ms: u32,
  speech_ms: u32,
}
