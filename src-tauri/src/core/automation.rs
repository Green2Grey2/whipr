use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use arboard::Clipboard;
use enigo::{Enigo, KeyboardControllable, Key};

use crate::core::runtime::{self, HelperAvailability, PasteMethod, SessionType};

#[derive(Debug)]
enum WaylandPasteHelper {
  Wtype,
  Ydotool,
}

pub fn paste_text(
  text: &str,
  delay_ms: u32,
  keep_clipboard: bool,
  paste_method: &str,
  focus_window_id: Option<&str>,
) -> Result<(), String> {
  let session = runtime::detect_session_type();
  let helpers = runtime::detect_helpers();

  if !keep_clipboard {
    maybe_focus_window(session, &helpers, focus_window_id);
    return paste_without_clipboard(text, delay_ms, paste_method, session, &helpers);
  }

  let resolution = runtime::resolve_paste_method(paste_method, session, &helpers);
  if !matches!(resolution.method, PasteMethod::ClipboardOnly | PasteMethod::Unavailable) {
    maybe_focus_window(session, &helpers, focus_window_id);
  }

  match resolution.method {
    PasteMethod::X11CtrlV => paste_x11(text, delay_ms, keep_clipboard),
    PasteMethod::WaylandWtype => paste_wayland(
      text,
      delay_ms,
      keep_clipboard,
      &helpers,
      WaylandPasteHelper::Wtype,
    ),
    PasteMethod::WaylandYdotool => paste_wayland(
      text,
      delay_ms,
      keep_clipboard,
      &helpers,
      WaylandPasteHelper::Ydotool,
    ),
    PasteMethod::ClipboardOnly => paste_clipboard_only(text, keep_clipboard, session, &helpers),
    PasteMethod::Unavailable => {
      let detail = if resolution.missing_helpers.is_empty() {
        "Paste method unavailable".to_string()
      } else {
        format!(
          "Missing helpers: {}",
          resolution.missing_helpers.join(", ")
        )
      };
      if keep_clipboard {
        if paste_clipboard_only(text, true, session, &helpers).is_ok() {
          return Ok(());
        }
      }
      Err(detail)
    }
  }
}

pub fn copy_text(text: &str) -> Result<(), String> {
  let session = runtime::detect_session_type();
  let helpers = runtime::detect_helpers();
  paste_clipboard_only(text, true, session, &helpers)
}

pub fn capture_focus_window() -> Option<String> {
  let session = runtime::detect_session_type();
  if session != SessionType::X11 {
    return None;
  }
  let helpers = runtime::detect_helpers();
  if !helpers.xdotool {
    return None;
  }

  let output = Command::new("xdotool").arg("getwindowfocus").output().ok()?;
  if !output.status.success() {
    return None;
  }
  let id = String::from_utf8_lossy(&output.stdout).trim().to_string();
  if id.is_empty() {
    None
  } else {
    Some(id)
  }
}

fn maybe_focus_window(
  session: SessionType,
  helpers: &HelperAvailability,
  focus_window_id: Option<&str>,
) {
  if session != SessionType::X11 || !helpers.xdotool {
    return;
  }
  if let Some(window_id) = focus_window_id {
    let _ = focus_x11(window_id);
  }
}

fn focus_x11(window_id: &str) -> Result<(), String> {
  let status = Command::new("xdotool")
    .args(["windowactivate", "--sync", window_id])
    .status()
    .map_err(|err| err.to_string())?;
  if status.success() {
    Ok(())
  } else {
    Err(format!("xdotool failed with status {status}"))
  }
}

fn paste_without_clipboard(
  text: &str,
  delay_ms: u32,
  paste_method: &str,
  session: SessionType,
  helpers: &HelperAvailability,
) -> Result<(), String> {
  match session {
    SessionType::X11 | SessionType::Windows | SessionType::Macos => type_x11(text, delay_ms),
    SessionType::Wayland => {
      let helper = resolve_wayland_type_helper(paste_method, helpers)?;
      type_wayland(text, delay_ms, helper)
    }
    SessionType::Unknown => Err("No display session detected".to_string()),
  }
}

fn resolve_wayland_type_helper(
  paste_method: &str,
  helpers: &HelperAvailability,
) -> Result<WaylandPasteHelper, String> {
  let normalized = paste_method.trim().to_lowercase();
  let request = if normalized.is_empty() {
    "auto"
  } else {
    normalized.as_str()
  };

  match request {
    "wayland_wtype" => helpers
      .wtype
      .then_some(WaylandPasteHelper::Wtype)
      .ok_or_else(|| "Missing helpers: wtype".to_string()),
    "wayland_ydotool" => helpers
      .ydotool
      .then_some(WaylandPasteHelper::Ydotool)
      .ok_or_else(|| "Missing helpers: ydotool".to_string()),
    "clipboard_only" => Err(
      "Clipboard-only paste is disabled when copy-to-clipboard is off".to_string(),
    ),
    "x11_ctrl_v" | "auto" | _ => {
      if helpers.wtype {
        Ok(WaylandPasteHelper::Wtype)
      } else if helpers.ydotool {
        Ok(WaylandPasteHelper::Ydotool)
      } else {
        Err("Missing helpers: wtype, ydotool".to_string())
      }
    }
  }
}

fn type_x11(text: &str, delay_ms: u32) -> Result<(), String> {
  if delay_ms > 0 {
    thread::sleep(Duration::from_millis(delay_ms as u64));
  }

  let mut enigo = Enigo::new();
  enigo.key_sequence(text);
  Ok(())
}

fn type_wayland(
  text: &str,
  delay_ms: u32,
  helper: WaylandPasteHelper,
) -> Result<(), String> {
  if delay_ms > 0 {
    thread::sleep(Duration::from_millis(delay_ms as u64));
  }

  match helper {
    WaylandPasteHelper::Wtype => send_wtype_text(text),
    WaylandPasteHelper::Ydotool => send_ydotool_text(text),
  }
}

fn paste_x11(text: &str, delay_ms: u32, keep_clipboard: bool) -> Result<(), String> {
  let mut clipboard = Clipboard::new().map_err(|err| err.to_string())?;
  let previous = if keep_clipboard {
    None
  } else {
    clipboard.get_text().ok()
  };

  clipboard
    .set_text(text.to_string())
    .map_err(|err| err.to_string())?;

  if delay_ms > 0 {
    thread::sleep(Duration::from_millis(delay_ms as u64));
  }

  let mut enigo = Enigo::new();
  let modifier = paste_modifier_key();
  enigo.key_down(modifier);
  enigo.key_click(Key::Layout('v'));
  enigo.key_up(modifier);

  if let Some(previous_text) = previous {
    let _ = clipboard.set_text(previous_text);
  }

  Ok(())
}

fn paste_wayland(
  text: &str,
  delay_ms: u32,
  keep_clipboard: bool,
  helpers: &HelperAvailability,
  helper: WaylandPasteHelper,
) -> Result<(), String> {
  if !helpers.wl_copy {
    return Err("wl-copy is required for Wayland clipboard support".to_string());
  }

  if !keep_clipboard && !helpers.wl_paste {
    return Err("wl-paste is required to restore the clipboard on Wayland".to_string());
  }

  let previous = if keep_clipboard {
    None
  } else {
    Some(wl_paste_text()?)
  };

  wl_copy_text(text)?;

  if delay_ms > 0 {
    thread::sleep(Duration::from_millis(delay_ms as u64));
  }

  match helper {
    WaylandPasteHelper::Wtype => send_wtype_paste()?,
    WaylandPasteHelper::Ydotool => send_ydotool_paste()?,
  }

  if let Some(previous_text) = previous {
    let _ = wl_copy_text(&previous_text);
  }

  Ok(())
}

fn paste_clipboard_only(
  text: &str,
  _keep_clipboard: bool,
  session: SessionType,
  helpers: &HelperAvailability,
) -> Result<(), String> {
  let mut wl_copy_error: Option<String> = None;

  if matches!(session, SessionType::Wayland) && helpers.wl_copy {
    match wl_copy_text(text) {
      Ok(()) => return Ok(()),
      Err(err) => wl_copy_error = Some(err),
    }
  }

  let arboard_result = set_clipboard_text(text);
  if arboard_result.is_ok() {
    return Ok(());
  }

  if let Some(err) = wl_copy_error {
    return Err(err);
  }

  if session == SessionType::Wayland && !helpers.wl_copy {
    return Err("wl-copy is required for Wayland clipboard support".to_string());
  }

  if session == SessionType::Unknown {
    return Err("No display session detected".to_string());
  }

  arboard_result
}

fn set_clipboard_text(text: &str) -> Result<(), String> {
  let mut clipboard = Clipboard::new().map_err(|err| err.to_string())?;
  clipboard
    .set_text(text.to_string())
    .map_err(|err| err.to_string())
}

fn wl_copy_text(text: &str) -> Result<(), String> {
  let mut child = Command::new("wl-copy")
    .stdin(Stdio::piped())
    .spawn()
    .map_err(|err| err.to_string())?;

  {
    let stdin = child
      .stdin
      .as_mut()
      .ok_or_else(|| "Failed to open wl-copy stdin".to_string())?;
    stdin.write_all(text.as_bytes()).map_err(|err| err.to_string())?;
  }

  let status = child.wait().map_err(|err| err.to_string())?;
  if !status.success() {
    return Err(format!("wl-copy failed with status {status}"));
  }

  Ok(())
}

fn wl_paste_text() -> Result<String, String> {
  let output = Command::new("wl-paste")
    .output()
    .map_err(|err| err.to_string())?;

  if !output.status.success() {
    return Err(format!("wl-paste failed with status {}", output.status));
  }

  String::from_utf8(output.stdout).map_err(|err| err.to_string())
}

fn send_wtype_paste() -> Result<(), String> {
  let status = Command::new("wtype")
    .args(["-M", "ctrl", "-k", "v", "-m", "ctrl"])
    .status()
    .map_err(|err| err.to_string())?;

  if status.success() {
    Ok(())
  } else {
    Err(format!("wtype failed with status {status}"))
  }
}

fn send_wtype_text(text: &str) -> Result<(), String> {
  if text.is_empty() {
    return Ok(());
  }

  let status = Command::new("wtype")
    .arg("--")
    .arg(text)
    .status()
    .map_err(|err| err.to_string())?;

  if status.success() {
    Ok(())
  } else {
    Err(format!("wtype failed with status {status}"))
  }
}

fn send_ydotool_paste() -> Result<(), String> {
  let status = Command::new("ydotool")
    .args(["key", "29:1", "47:1", "47:0", "29:0"])
    .status()
    .map_err(|err| err.to_string())?;

  if status.success() {
    Ok(())
  } else {
    Err(format!("ydotool failed with status {status}"))
  }
}

fn send_ydotool_text(text: &str) -> Result<(), String> {
  if text.is_empty() {
    return Ok(());
  }

  let status = Command::new("ydotool")
    .args(["type", "--", text])
    .status()
    .map_err(|err| err.to_string())?;

  if status.success() {
    Ok(())
  } else {
    Err(format!("ydotool failed with status {status}"))
  }
}

fn paste_modifier_key() -> Key {
  #[cfg(target_os = "macos")]
  {
    Key::Meta
  }

  #[cfg(not(target_os = "macos"))]
  {
    Key::Control
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::core::runtime::HelperAvailability;

  #[test]
  fn wayland_helper_prefers_wtype() {
    let helpers = HelperAvailability {
      wl_copy: true,
      wl_paste: true,
      wtype: true,
      ydotool: true,
      xdotool: false,
    };
    let helper = resolve_wayland_type_helper("auto", &helpers).expect("helper");
    assert!(matches!(helper, WaylandPasteHelper::Wtype));
  }

  #[test]
  fn wayland_helper_falls_back_to_ydotool() {
    let helpers = HelperAvailability {
      wl_copy: true,
      wl_paste: true,
      wtype: false,
      ydotool: true,
      xdotool: false,
    };
    let helper = resolve_wayland_type_helper("auto", &helpers).expect("helper");
    assert!(matches!(helper, WaylandPasteHelper::Ydotool));
  }

  #[test]
  fn wayland_helper_errors_when_missing() {
    let helpers = HelperAvailability {
      wl_copy: true,
      wl_paste: true,
      wtype: false,
      ydotool: false,
      xdotool: false,
    };
    let err = resolve_wayland_type_helper("auto", &helpers).unwrap_err();
    assert!(err.contains("wtype"));
  }

  #[test]
  fn wayland_helper_respects_specific_request() {
    let helpers = HelperAvailability {
      wl_copy: true,
      wl_paste: true,
      wtype: true,
      ydotool: true,
      xdotool: false,
    };
    let helper = resolve_wayland_type_helper("wayland_ydotool", &helpers).expect("helper");
    assert!(matches!(helper, WaylandPasteHelper::Ydotool));
  }
}
