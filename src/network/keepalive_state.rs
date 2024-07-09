use chrono::Local;

#[derive(Debug)]
pub struct KeepAliveState {
    pub keepalive_pending: bool,
    pub last_keepalive_time: i64,
}

impl KeepAliveState {
    pub fn new() -> Self {
        Self {
            keepalive_pending: false,
            last_keepalive_time: Local::now().timestamp_millis(),
        }
    }
}
