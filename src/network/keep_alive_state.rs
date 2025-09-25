use chrono::Local;

#[derive(Debug)]
pub struct KeepAliveState {
    pub pending: bool,
    pub last_keep_alive_time: i64,
}

impl Default for KeepAliveState {
    fn default() -> Self {
        Self {
            pending: true,
            last_keep_alive_time: Local::now().timestamp_millis(),
        }
    }
}
