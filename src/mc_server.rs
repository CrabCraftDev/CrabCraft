use std::ops::Add;
use std::time::Duration;

use log::{info, warn};
use tokio::time::{sleep_until, Instant};

use crate::network::tcp_server::TcpServer;
use crate::utils::scheduler::ServerScheduler;

const TPS: u64 = 20;
const MAX_TICK_DURATION: Duration = Duration::from_millis(1000 / TPS);
const SERVER_OVERLOAD_WARNING_COOLDOWN: Duration = Duration::from_secs(20);

#[allow(dead_code)]
pub struct McServer<'a> {
    tcp_server: &'a TcpServer,
    running: bool,
    current_tick: u64,
    scheduler: ServerScheduler,
    next_tick_start: Instant,
    last_overload_warning: Instant,
}

impl McServer<'_> {
    pub fn new(tcp_server: &TcpServer) -> McServer {
        McServer {
            tcp_server,
            running: true,
            current_tick: 0,
            scheduler: ServerScheduler::default(),
            next_tick_start: Instant::now(),
            last_overload_warning: Instant::now().checked_sub(SERVER_OVERLOAD_WARNING_COOLDOWN).unwrap(),
        }
    }

    pub async fn tick_loop(&mut self) {
        while self.running {
            let last_tick_time = Instant::now() - self.next_tick_start + MAX_TICK_DURATION;
            if last_tick_time.as_millis() > MAX_TICK_DURATION.as_millis() + 5
                && self.last_overload_warning.add(SERVER_OVERLOAD_WARNING_COOLDOWN) < Instant::now()
            {
                warn!(
                    "Can't keep up! Is the server overloaded? The last tick took {}ms, while max tick time is {}ms!",
                    last_tick_time.as_millis(),
                    MAX_TICK_DURATION.as_millis()
                );
                self.last_overload_warning = Instant::now();
            }
            self.next_tick_start = Instant::now().add(MAX_TICK_DURATION);

            self.tick();

            sleep_until(self.next_tick_start).await;
        }
    }
    pub fn tick(&mut self) {
        self.current_tick += 1;
        self.scheduler.tick();
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
