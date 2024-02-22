use std::time::Duration;

use chrono::Local;
use log::{info, warn};
use tokio::time::sleep;

use crate::network::tcp_server::TcpServer;
use crate::utils::scheduler::ServerScheduler;

const TPS: u64 = 20;
const MAX_TICK_DURATION: u64 = 1000 / TPS;

#[allow(dead_code)]
pub struct McServer<'a> {
    tcp_server: &'a TcpServer,
    running: bool,
    current_tick: u64,
    scheduler: ServerScheduler,
}

impl McServer<'_> {
    pub fn new(tcp_server: &TcpServer) -> McServer {
        McServer {
            tcp_server,
            running: true,
            current_tick: 0,
            scheduler: ServerScheduler::default(),
        }
    }

    pub async fn tick_loop(&mut self) {
        while self.running {
            self.current_tick += 1;
            let tick_start = Local::now().timestamp_millis() as u64;

            self.tick();

            let tick_end = Local::now().timestamp_millis() as u64;
            let tick_duration = tick_end - tick_start;
            if tick_duration > MAX_TICK_DURATION {
                warn!(
                    "Can't keep up! Is the server overloaded? The last tick took {}ms, while max is {}ms!",
                    tick_duration, MAX_TICK_DURATION
                );
                continue;
            }
            sleep(Duration::from_millis(MAX_TICK_DURATION - tick_duration)).await;
        }
    }
    pub fn tick(&mut self) {
        self.scheduler.tick()
    }
    #[allow(dead_code)]
    pub fn shutdown(&mut self) {
        info!("Stopping the server");
        self.running = false;
    }
    #[allow(clippy::needless_return)]
    pub(crate) fn do_shutdown(&mut self) {
        if self.running {
            warn!("There was an attempt to process shutdown logic while still running!");
            return;
        }
    }
}
