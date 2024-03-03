use std::ops::Add;
use std::sync::Arc;
use std::time::Duration;

use log::{info, warn};
use tokio::time::{sleep_until, Instant};
use tokio::{select, signal};

use crate::network::tcp_server::TcpServer;
use crate::utils::scheduler::ServerScheduler;

const TPS: u64 = 20;
const MAX_TICK_DURATION: Duration = Duration::from_millis(1000 / TPS);
const SERVER_OVERLOAD_WARNING_COOLDOWN: Duration = Duration::from_secs(20);

pub struct McServer {
    running: bool,
    current_tick: u64,
    scheduler: ServerScheduler,
    tcp_server: Arc<TcpServer>,
    next_tick_start: Instant,
    last_overload_warning: Instant,
}

impl McServer {
    pub fn new(ip_address: String) -> McServer {
        let tcp_server = TcpServer::new(ip_address);
        McServer {
            running: true,
            current_tick: 0,
            tcp_server: Arc::new(tcp_server),
            scheduler: ServerScheduler::default(),
            next_tick_start: Instant::now(),
            last_overload_warning: Instant::now().checked_sub(SERVER_OVERLOAD_WARNING_COOLDOWN).unwrap(),
        }
    }

    pub async fn start(mut self) {
        let tcp_server_arc = Arc::clone(&self.tcp_server);
        let tcp_server_task = {
            tokio::spawn(async move {
                tcp_server_arc.start().await;
            })
        };

        let ctrl_c = signal::ctrl_c();

        select! {
            _ = ctrl_c => {
                println!(); // To write new messages after ^C
                self.shutdown();
            },
            _ = tcp_server_task => {},
            _ = self.tick_loop() => {},
        }

        self.do_shutdown().await;
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

            self.tick().await;

            sleep_until(self.next_tick_start).await;
        }
    }

    pub async fn tick(&mut self) {
        self.current_tick += 1;
        self.scheduler.tick();

        self.tcp_server.tick().await;
    }

    pub fn shutdown(&mut self) {
        info!("Disabling the server...");
        self.running = false;
    }

    async fn do_shutdown(&mut self) {
        info!("Stopping server...");
        if self.running {
            warn!("Stopping server while it is still running!");
            return;
        }

        let connections = &self.tcp_server.connections.lock().await;
        for connection in connections.iter() {
            let mut connection = connection.lock().await;
            connection.disconnect("Server shutting down!").await;
        }

        info!("CrabCraft stopped! Bye!")
    }
}
