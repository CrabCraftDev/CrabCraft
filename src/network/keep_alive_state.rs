use chrono::Local;

#[derive(Debug)]
pub struct KeepAliveState {
    pub keep_alive_pending: bool,
    pub last_keep_alive_time: i64,
}

impl KeepAliveState {
    pub fn new() -> Self {
        Self {
            keep_alive_pending: false,
            last_keep_alive_time: Local::now().timestamp_millis(),
        }
    }
}
