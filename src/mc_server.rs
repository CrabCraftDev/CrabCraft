use evenio::world::World;
use kanal::{AsyncReceiver, AsyncSender};
use std::ops::Add;
use std::panic;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{self, Instant, MissedTickBehavior};
use tokio::{select, signal};
use tracing::{error, info, info_span, warn};

use crate::events::asynchronous::{AsyncEvent, AsyncEventHolder};
use crate::events::global::server::ServerShutdownEvent;
use crate::events::global::world::WorldTickEvent;
use crate::handlers::init_handlers::init_handlers;
use crate::utils::scheduler::ServerScheduler;
use crate::{
    network::tcp_server::TcpServer,
    utils::errors::{print_error_report_instructions, print_panic},
};

pub const SERVER_PROTOCOL_VERSION: i32 = 769;
pub const SERVER_VERSION: &str = "1.21.4";
pub const COMPATIBLE_SERVER_VERSIONS: &[&str] = &["1.21.4"];

const TPS: u64 = 20;
const MAX_TICK_DURATION: Duration = Duration::from_millis(1000 / TPS);
const SERVER_OVERLOAD_WARNING_COOLDOWN: Duration = Duration::from_secs(20);

pub struct McServer {
    running: bool,
    current_tick: u64,
    scheduler: ServerScheduler,
    tcp_server: Arc<TcpServer>,
    world: World,
    /// Channels used for communication between the main thread and others
    events_receiver: AsyncReceiver<AsyncEventHolder>,
    event_sender: AsyncSender<AsyncEventHolder>,
}

impl McServer {
    pub fn new(ip_address: String) -> McServer {
        let (event_sender, events_receiver) = kanal::unbounded_async::<AsyncEventHolder>();
        let tcp_server = TcpServer::new(ip_address, event_sender.clone());

        let mut world = World::new();
        init_handlers(&mut world);

        McServer {
            running: true,
            current_tick: 0,
            tcp_server: Arc::new(tcp_server),
            scheduler: ServerScheduler::default(),
            world,
            events_receiver,
            event_sender,
        }
    }

    #[tracing::instrument(name = "Mc Server", skip_all)]
    pub async fn start(mut self) {
        let ctrl_c = signal::ctrl_c();

        let event_channel = self.event_sender.clone();
        select! {
            _ = ctrl_c => {
                println!(); // To write new messages after ^C
                self.shutdown();
            },
            _ = McServer::tcp_server_task(Arc::clone(&self.tcp_server)) => {},
            _ = Self::tick_loop(event_channel) => {},
            _ = self.event_loop() => {}
        }

        self.do_shutdown().await;
    }

    async fn tcp_server_task(tcp_server: Arc<TcpServer>) {
        // We are creating task and then awaiting it, to handle panic errors
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

    #[tracing::instrument(name = "Tick Scheduler", skip_all)]
    pub async fn tick_loop(event_sender: AsyncSender<AsyncEventHolder>) {
        let mut ticker = time::interval(Duration::from_millis(1000 / TPS));
        let mut overload_warning_cooldown = Instant::now();
        ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);

        loop {
            let tick_start = Instant::now();

            AsyncEvent::send_event_with_response(WorldTickEvent, &event_sender)
                .await
                .unwrap();

            ticker.tick().await;

            let elapsed_time = tick_start.elapsed().saturating_sub(MAX_TICK_DURATION);
            if elapsed_time > MAX_TICK_DURATION && overload_warning_cooldown < Instant::now() {
                warn!(
                    "Can't keep up! Is the server overloaded? The last tick took {}ms, while max tick time is {}ms!",
                    elapsed_time.as_millis(),
                    MAX_TICK_DURATION.as_millis()
                );
                overload_warning_cooldown = Instant::now().add(SERVER_OVERLOAD_WARNING_COOLDOWN);
            }
        }
    }

    #[tracing::instrument(name = "Event Loop", skip_all)]
    pub async fn event_loop(&mut self) {
        while self.running {
            let Ok(event) = self.events_receiver.recv().await else {
                break;
            };

            let AsyncEventHolder {
                event,
                response_channel,
                entity_id,
            } = event;

            let event: Option<AsyncEvent> = match event {
                AsyncEvent::PlayerLogin(event) => self.world.send_with_response(event).map(AsyncEvent::PlayerLogin),
                AsyncEvent::PlayerPreLogin(event) => {
                    self.world.send_with_response(event).map(AsyncEvent::PlayerPreLogin)
                }
                AsyncEvent::ServerListPing(event) => {
                    self.world.send_with_response(event).map(AsyncEvent::ServerListPing)
                }
                AsyncEvent::PlayerQuit(event) => self
                    .world
                    .send_to_with_response(entity_id.expect("Missing entity for Quit event"), event)
                    .map(AsyncEvent::PlayerQuit),
                AsyncEvent::Packet(event) => self
                    .world
                    .send_to_with_response(entity_id.expect("Missing entity for Packet event"), event)
                    .map(AsyncEvent::Packet),
                AsyncEvent::WorldTick(event) => {
                    let span = info_span!("Tick");
                    let _span_guard = span.enter();
                    self.current_tick += 1;
                    self.scheduler.tick();
                    self.world.send_with_response(event).map(AsyncEvent::WorldTick)
                }
            };

            if let Some(response_channel) = response_channel {
                let Some(event) = event else {
                    panic!("AsyncEvent required response, but it was consumed by one of handlers!")
                };
                response_channel
                    .send(event)
                    .expect("Response channel for event was closed");
            }
        }
    }

    pub fn shutdown(&mut self) {
        self.running = false;
    }

    #[tracing::instrument(name = "Shutdown", skip_all)]
    async fn do_shutdown(&mut self) {
        info!("Stopping server...");
        if self.running {
            warn!("Stopping server while it is still running!");
            return;
        }

        self.world.send(ServerShutdownEvent);

        info!("CrabCraft has stopped! Bye!")
    }
}
