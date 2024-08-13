use chrono::Local;

#[derive(Debug)]
pub struct KeepAliveState {
    pub keep_alive_pending: bool,
    pub last_keepalive_time: i64,
}

impl KeepAliveState {
    pub fn new() -> Self {
        Self {
            keep_alive_pending: false,
            last_keepalive_time: Local::now().timestamp_millis(),
        }
    }
}
