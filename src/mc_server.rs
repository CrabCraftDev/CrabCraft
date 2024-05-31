use std::ops::Add;
use std::sync::Arc;
use std::time::Duration;

use evenio::world::World;
use kanal::Receiver;
use log::{info, warn};
use tokio::time::{sleep_until, Instant};
use tokio::{select, signal};

use crate::{
    entity::player::{network::connection::PlayerConnection, Player},
    network::tcp_server::TcpServer,
    world,
};
use crate::{utils::scheduler::ServerScheduler, world::WorldTick};

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
    player_connection_receiver: Receiver<PlayerConnection>,
}

impl McServer {
    pub fn new(ip_address: String) -> McServer {
        let (player_connection_sender, player_connection_receiver) = kanal::unbounded();
        let tcp_server = TcpServer::new(ip_address, player_connection_sender);

        McServer {
            running: true,
            current_tick: 0,
            tcp_server: Arc::new(tcp_server),
            scheduler: ServerScheduler::default(),
            overload_warning_cooldown: Instant::now(),
            world: world::new(),
            player_connection_receiver,
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
            let tick_start = Instant::now();
            self.tick().await;
            let last_tick_duration = Instant::now() - tick_start;

            sleep_until(tick_start.add(MAX_TICK_DURATION)).await;

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

        while let Some(connection) = self.player_connection_receiver.try_recv().unwrap() {
            let entity = self.world.spawn();
            self.world.insert(entity, Player);
            self.world.insert(entity, connection);
        }

        self.world.send(WorldTick);

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
