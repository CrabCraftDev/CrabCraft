use std::ops::Add;
use std::sync::Arc;
use std::time::Duration;

use log::{info, warn};
use tokio::time::{sleep_until, Instant};

use crate::network::tcp_server::TcpServer;
use crate::utils::scheduler::ServerScheduler;

const TPS: u64 = 20;
const MAX_TICK_DURATION: Duration = Duration::from_millis(1000 / TPS);
const SERVER_OVERLOAD_WARNING_COOLDOWN: Duration = Duration::from_secs(20);

pub struct McServer {
    running: bool,
    current_tick: u64,
    scheduler: ServerScheduler,
    next_tick_start: Instant,
    last_overload_warning: Instant,
}

impl McServer {
    pub fn new() -> McServer {
        McServer {
            running: true,
            current_tick: 0,
            scheduler: ServerScheduler::default(),
            next_tick_start: Instant::now(),
            last_overload_warning: Instant::now().checked_sub(SERVER_OVERLOAD_WARNING_COOLDOWN).unwrap(),
        }
    }

    pub async fn start(mut self, ip_address: String) {
        let tcp_server = TcpServer::new(ip_address);
        let tcp_server_arc = Arc::new(tcp_server);

        let tcp_server_task = {
            let tcp_server_arc = Arc::clone(&tcp_server_arc);
            tokio::spawn(async move {
                tcp_server_arc.start().await;
            })
        };

        self.tick_loop(tcp_server_arc).await;
        tcp_server_task.await.unwrap();

        self.shutdown();
    }

    pub async fn tick_loop(&mut self, tcp_server: Arc<TcpServer>) {
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

            self.tick(tcp_server.clone()).await;

            sleep_until(self.next_tick_start).await;
        }
    }

    pub async fn tick(&mut self, _tcp_server: Arc<TcpServer>) {
        self.current_tick += 1;
        self.scheduler.tick();
    }

    #[allow(dead_code)]
    pub fn shutdown(&mut self) {
        info!("Stopping the server");
        self.running = false;
    }

    #[allow(clippy::needless_return)]
    #[allow(dead_code)]
    pub(crate) fn do_shutdown(&mut self) {
        if self.running {
            warn!("There was an attempt to process shutdown logic while still running!");
            return;
        }
    }
}
