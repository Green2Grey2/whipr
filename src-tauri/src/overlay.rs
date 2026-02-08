#[cfg(target_os = "linux")]
mod linux_overlay {
    use std::fs;
    use std::path::PathBuf;
    use std::sync::{Mutex, OnceLock};
    use std::time::{SystemTime, UNIX_EPOCH};

    use serde::Serialize;

    const STATE_DIR: &str = "whispr";
    const STATE_FILE: &str = "overlay.json";

    // Reduce filesystem churn: coalesce frequent meter updates.
    //
    // The GNOME extension considers state stale after 5s; keepalive at 1s keeps it fresh while
    // dropping writes from ~8/sec to ~1/sec in steady-state (and more when level changes).
    const OVERLAY_WRITE_MIN_INTERVAL_MS: i64 = 250;
    const OVERLAY_WRITE_MAX_INTERVAL_MS: i64 = 1_000;
    const OVERLAY_LEVEL_QUANTIZE_STEPS: i32 = 100;
    const OVERLAY_LEVEL_MIN_DELTA_STEPS: i32 = 2; // ~= 2%

    #[derive(Clone, Copy, Debug, Default)]
    struct OverlayWriteCache {
        initialized: bool,
        last_recording: bool,
        last_started_at_ms: Option<i64>,
        last_level_q: i32,
        last_write_at_ms: i64,
    }

    static OVERLAY_CACHE: OnceLock<Mutex<OverlayWriteCache>> = OnceLock::new();

    #[derive(Serialize)]
    struct OverlayState {
        recording: bool,
        started_at_ms: Option<i64>,
        updated_at_ms: i64,
        level: Option<f32>,
    }

    fn now_ms() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis() as i64)
            .unwrap_or(0)
    }

    fn quantize_level(level: Option<f32>) -> i32 {
        let Some(level) = level else {
            return -1;
        };
        if !level.is_finite() {
            return -1;
        }
        let clamped = level.clamp(0.0, 1.0);
        ((clamped * OVERLAY_LEVEL_QUANTIZE_STEPS as f32).round() as i32)
            .clamp(0, OVERLAY_LEVEL_QUANTIZE_STEPS)
    }

    fn should_write(
        cache: &OverlayWriteCache,
        recording: bool,
        started_at_ms: Option<i64>,
        level_q: i32,
        now_ms: i64,
    ) -> bool {
        if !cache.initialized {
            return true;
        }
        if cache.last_recording != recording {
            return true;
        }
        if cache.last_started_at_ms != started_at_ms {
            return true;
        }
        if !recording {
            // Not recording: state isn't time-sensitive; avoid periodic writes.
            return cache.last_level_q != level_q;
        }

        let since = now_ms.saturating_sub(cache.last_write_at_ms);
        if since >= OVERLAY_WRITE_MAX_INTERVAL_MS {
            return true; // keepalive
        }
        if since < OVERLAY_WRITE_MIN_INTERVAL_MS {
            return false;
        }

        (cache.last_level_q - level_q).abs() >= OVERLAY_LEVEL_MIN_DELTA_STEPS
    }

    fn state_dir() -> PathBuf {
        if let Some(dir) = std::env::var_os("XDG_STATE_HOME") {
            return PathBuf::from(dir).join(STATE_DIR);
        }
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home).join(".local/state").join(STATE_DIR);
        }
        std::env::temp_dir().join(STATE_DIR)
    }

    fn state_path() -> PathBuf {
        state_dir().join(STATE_FILE)
    }

    pub fn write_state(
        recording: bool,
        started_at_ms: Option<i64>,
        level: Option<f32>,
    ) -> Result<(), String> {
        let now = now_ms();
        let level_q = quantize_level(level);

        let cache_lock = OVERLAY_CACHE.get_or_init(|| Mutex::new(OverlayWriteCache::default()));
        {
            let cache = cache_lock
                .lock()
                .map_err(|_| "overlay cache lock poisoned".to_string())?;
            if !should_write(&cache, recording, started_at_ms, level_q, now) {
                return Ok(());
            }
        }

        let path = state_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|err| err.to_string())?;
        }

        let state = OverlayState {
            recording,
            started_at_ms,
            updated_at_ms: now,
            level,
        };
        let payload = serde_json::to_string(&state).map_err(|err| err.to_string())?;
        let tmp_path = path.with_extension("tmp");
        fs::write(&tmp_path, payload).map_err(|err| err.to_string())?;
        fs::rename(&tmp_path, &path).map_err(|err| err.to_string())?;

        if let Ok(mut cache) = cache_lock.lock() {
            *cache = OverlayWriteCache {
                initialized: true,
                last_recording: recording,
                last_started_at_ms: started_at_ms,
                last_level_q: level_q,
                last_write_at_ms: now,
            };
        }

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn overlay_write_state_throttles_stable_meter_updates() {
            let cache = OverlayWriteCache {
                initialized: true,
                last_recording: true,
                last_started_at_ms: Some(1),
                last_level_q: 50,
                last_write_at_ms: 1000,
            };

            // Too soon: no write even if level changes a bit.
            assert!(!should_write(&cache, true, Some(1), 52, 1100));

            // After min interval but below delta: no write.
            assert!(!should_write(&cache, true, Some(1), 51, 1300));

            // After min interval and delta exceeded: write.
            assert!(should_write(&cache, true, Some(1), 55, 1300));

            // Keepalive.
            assert!(should_write(&cache, true, Some(1), 50, 2100));
        }

        #[test]
        fn overlay_write_state_always_writes_on_state_change() {
            let cache = OverlayWriteCache {
                initialized: true,
                last_recording: false,
                last_started_at_ms: None,
                last_level_q: -1,
                last_write_at_ms: 1000,
            };
            assert!(should_write(&cache, true, Some(123), 0, 1010));
        }
    }
}

#[cfg(target_os = "linux")]
pub use linux_overlay::write_state;

#[cfg(not(target_os = "linux"))]
pub fn write_state(
    _recording: bool,
    _started_at_ms: Option<i64>,
    _level: Option<f32>,
) -> Result<(), String> {
    Ok(())
}
