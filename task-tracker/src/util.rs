use std::time::{SystemTime, UNIX_EPOCH};

pub fn cur_time_ms_u128() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or_default()
}
