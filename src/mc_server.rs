use crate::network::connection::Connection;
use crate::world::Shutdown;
use crate::{entity::player::KeepAliveStatus, network::tcp_server::TcpServer, world};
use crate::{utils::scheduler::ServerScheduler, world::WorldTick};
use chrono::Local;
use evenio::world::World;
use kanal::Receiver;
use log::{info, warn};
use std::ops::Add;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{self, Instant, MissedTickBehavior};
use tokio::{select, signal};

pub const SERVER_PROTOCOL_VERSION: i32 = 767;
pub const SERVER_VERSION: &str = "1.21.1";
pub const COMPATIBLE_SERVER_VERSIONS: &[&str] = &["1.21", "1.21.1"];

const TPS: u64 = 20;
const MAX_TICK_DURATION: Duration = Duration::from_millis(1000 / TPS);
const SERVER_OVERLOAD_WARNING_COOLDOWN: Duration = Duration::from_secs(20);

pub struct McServer {
    running: bool,
    current_tick: u64,
    scheduler: ServerScheduler,
    tcp_server: Arc<TcpServer>,
    overload_warning_cooldown: Instant,
    world: World,
    incoming_connections_receiver: Receiver<Connection>,
}

impl McServer {
    pub fn new(ip_address: String) -> McServer {
        let (incoming_connections_sender, incoming_connections_receiver) = kanal::unbounded();
        let tcp_server = TcpServer::new(ip_address, incoming_connections_sender);

        McServer {
            running: true,
            current_tick: 0,
            tcp_server: Arc::new(tcp_server),
            scheduler: ServerScheduler::default(),
            overload_warning_cooldown: Instant::now(),
            world: world::new(),
            incoming_connections_receiver,
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
                println!(); // Write new messages after ^C
                self.shutdown();
            },
            _ = tcp_server_task => {},
            _ = self.tick_loop() => {},
        }

        self.do_shutdown().await;
    }

    pub async fn tick_loop(&mut self) {
        let mut ticker = time::interval(Duration::from_millis(1000 / TPS));
        ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);

        while self.running {
            let tick_start = Instant::now();
            self.tick().await;
            let last_tick_duration = Instant::now() - tick_start;

            ticker.tick().await;

            if last_tick_duration > MAX_TICK_DURATION && self.overload_warning_cooldown < Instant::now() {
                warn!(
                    "Can't keep up! Is the server overloaded? The last tick took {}ms, while max tick time is {}ms!",
                    last_tick_duration.as_millis(),
                    MAX_TICK_DURATION.as_millis()
                );
                self.overload_warning_cooldown = Instant::now().add(SERVER_OVERLOAD_WARNING_COOLDOWN);
            }
        }
    }

    pub async fn tick(&mut self) {
        self.current_tick += 1;

        while let Some(new_connection) = self.incoming_connections_receiver.try_recv().unwrap() {
            let entity = self.world.spawn();
            self.world.insert(
                entity,
                KeepAliveStatus {
                    keepalive_pending: false,
                    last_keepalive_time: Local::now().timestamp_millis(),
                },
            );
            self.world.insert(entity, new_connection);
        }

        self.world.send(WorldTick);

        self.scheduler.tick();
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

        self.world.send(Shutdown);

        info!("CrabCraft stopped! Bye!")
    }
}
