use std::ops::Add;
use std::sync::Arc;
use std::time::Duration;

use evenio::world::World;
use kanal::Receiver;
use tokio::time::{self, Instant, MissedTickBehavior};
use tokio::{select, signal};
use tracing::{error, info, warn};

use crate::{
    entity::player::network::incoming_connection::IncomingConnection,
    network::tcp_server::TcpServer,
    utils::errors::{print_error_report_instructions, print_panic},
    world::{self, ServerShutdown},
};
use crate::{utils::scheduler::ServerScheduler, world::WorldTick};

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
    incoming_connection_receiver: Receiver<IncomingConnection>,
}

impl McServer {
    pub fn new(ip_address: String) -> McServer {
        let (incoming_connection_sender, incoming_connection_receiver) = kanal::unbounded();
        let tcp_server = TcpServer::new(ip_address, incoming_connection_sender);

        McServer {
            running: true,
            current_tick: 0,
            tcp_server: Arc::new(tcp_server),
            scheduler: ServerScheduler::default(),
            overload_warning_cooldown: Instant::now(),
            world: world::new(),
            incoming_connection_receiver,
        }
    }

    pub async fn start(mut self) {
        let ctrl_c = signal::ctrl_c();

        select! {
            _ = ctrl_c => {
                println!(); // To write new messages after ^C
                self.shutdown();
            },
            _ = McServer::tcp_server_task(Arc::clone(&self.tcp_server)) => {},
            _ = self.tick_loop() => {},
        }

        self.do_shutdown().await;
    }

    async fn tcp_server_task(tcp_server: Arc<TcpServer>) {
        let task = tokio::spawn(async move {
            tcp_server.start().await;
        });
        let Err(err) = task.await else {
            return;
        };
        if !err.is_panic() {
            return;
        }
        error!("");
        error!("TcpServer panicked!");
        error!("");
        print_panic(err.into_panic());
        print_error_report_instructions();
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

        while let Some(incoming_connection) = self.incoming_connection_receiver.try_recv().unwrap() {
            self.world.send(incoming_connection);
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

        self.world.send(ServerShutdown);

        info!("CrabCraft stopped! Bye!")
    }
}
