use std::mem::take;
use evenio::world::World;
use kanal::{AsyncReceiver, AsyncSender};
use std::ops::Add;
use std::panic;
use std::process::exit;
use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::atomic::Ordering::SeqCst;
use std::time::Duration;
use tokio::time::{self, Instant, MissedTickBehavior};
use tokio::{join, select, signal};
use tokio::sync::Mutex;
use tokio::task::{LocalSet};
use tracing::{error, info, warn};

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
    running: Arc<AtomicBool>,
    current_tick: u64,
    scheduler: ServerScheduler,
    tcp_server: Arc<TcpServer>,
    /// Channels used for communication between the main thread and others
    events_receiver: AsyncReceiver<AsyncEventHolder>,
    event_sender: AsyncSender<AsyncEventHolder>,
    main_thread_status: Arc<Mutex<bool>>,
}

impl McServer {
    pub fn new(ip_address: String) -> McServer {
        let (event_sender, events_receiver) = kanal::unbounded_async::<AsyncEventHolder>();
        let tcp_server = TcpServer::new(ip_address, event_sender.clone());

        McServer {
            running: Arc::new(AtomicBool::new(true)),
            current_tick: 0,
            tcp_server: Arc::new(tcp_server),
            scheduler: ServerScheduler::default(),
            events_receiver,
            event_sender,
            main_thread_status: Arc::new(Mutex::new(true)),
        }
    }

    pub async fn start(mut self) {
        let ctrl_c = signal::ctrl_c();

        let tcp_server = Arc::clone(&self.tcp_server);
        let event_channel = self.event_sender.clone();


        let tcp_server = Arc::clone(&self.tcp_server);
        let event_channel = self.event_sender.clone();
        let event_channel2 = self.event_sender.clone();
        let main_thread_status = Arc::clone(&self.main_thread_status);


        // let afc = Arc::clone(&self.main_loop_working);
        //
        // // Spawn the event loop in the LocalSet
        // let a = local_set.spawn_local(async move {
        //     self.event_loop().await;
        //     self
        // });

        let skibidi = Arc::clone(&self.running);
        let skibidi2 = Arc::clone(&self.running);
        let skibidi3 = Arc::clone(&self.running);

        // use tokio_util::task::TaskTracker;
        // let tracker = TaskTracker::new();

        let main_task = tokio::task::spawn_blocking(move || {
            self.event_loop()
        });
        
        // let s = JoinSet::new();
        let abort = main_task.abort_handle();

        let a = join!(
            tokio::task::spawn(async move {
                signal::ctrl_c().await.unwrap();
                skibidi.store(false, Ordering::SeqCst);
                println!("Stopping main thread");
                AsyncEvent::send_event(ServerShutdownEvent, &event_channel2).expect("TODO: panic message");
                println!("FINISHED, GOODBYE");
                
                
                let abcd = tokio::task::spawn(async {
                    time::sleep(tokio::time::Duration::from_secs(5)).await;
                    println!("Main task didn't finish in 5 seconds");
                    println!("If you want press ctr c again to completly stop it");
                    println!("IT WON'T SAVE ANY DATA AND MAY CAUSE CORUPTION");
                    signal::ctrl_c().await.unwrap();
                    println!("Are you sure about that?");
                    signal::ctrl_c().await.unwrap();
                    println!("YOU KILLED ME");
                    exit(5);
                });
                
                
                select! {
                    _ = abcd => {
                        println!("BAJOJAJO");
                        
                    }
                    _ = main_task => {}
                }
            }),
            McServer::tcp_server_task(tcp_server, skibidi3),
            Self::tick_loop(event_channel, skibidi2),
            // main_task,
        );
        
        println!("BAJO JAJO");

        // *self = interface;

        // println!("skibidi {:?}", self);

        // self.do_shutdown().await;
    }

    async fn tcp_server_task(tcp_server: Arc<TcpServer>, is_disabled: Arc<AtomicBool>) {
        // We are creating task and then awaiting it, to handle panic errors
        let task = tokio::spawn(async move {
            tcp_server.start(is_disabled).await;
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

    pub async fn tick_loop(event_sender: AsyncSender<AsyncEventHolder>, is_running: Arc<AtomicBool>) {
        let mut ticker = time::interval(Duration::from_millis(1000 / TPS));
        let mut overload_warning_cooldown = Instant::now();
        ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);

        while is_running.load(SeqCst) {
            let tick_start = Instant::now();

            AsyncEvent::send_event(WorldTickEvent, &event_sender)
                // .await
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

    pub fn event_loop(mut self) {
        println!("a");
        let mut world = World::new();
        init_handlers(&mut world);

        while self.running.load(SeqCst) {
            println!("ev");
            let Ok(event) = self.events_receiver.as_sync().recv() else {
                break;
            };

            let AsyncEventHolder {
                event,
                response_channel,
                entity_id,
            } = event;

            let event: Option<AsyncEvent> = match event {
                AsyncEvent::PlayerLogin(event) => world.send_with_response(event).map(AsyncEvent::PlayerLogin),
                AsyncEvent::PlayerPreLogin(event) => {
                    world.send_with_response(event).map(AsyncEvent::PlayerPreLogin)
                }
                AsyncEvent::ServerListPing(event) => {
                    world.send_with_response(event).map(AsyncEvent::ServerListPing)
                }
                AsyncEvent::PlayerQuit(event) => world
                    .send_to_with_response(entity_id.expect("Missing entity for Quit event"), event)
                    .map(AsyncEvent::PlayerQuit),
                AsyncEvent::Packet(event) => world
                    .send_to_with_response(entity_id.expect("Missing entity for Packet event"), event)
                    .map(AsyncEvent::Packet),
                AsyncEvent::WorldTick(event) => {
                    self.current_tick += 1;
                    self.scheduler.tick();
                    
                    // todo: catch panics

                    // let world_ref = &mut self.world;
                    // let mut interface = take(world_ref);
                    // let mut world = self.world;
                    // let a = tokio::task::spawn_blocking(move || {
                    let b = world.send_with_response(event).map(AsyncEvent::WorldTick);
                    std::thread::sleep_ms(10000);
                    // (b, interface)
                    // });
                    // let o = a.await.unwrap();
                    // *world_ref = o.1;
                    // self = o.1;

                    b
                    // self = a.await.unwrap();

                },
                AsyncEvent::ServerShutdown(event) => {
                    world.send_with_response(event).map(AsyncEvent::ServerShutdown);
                    break;
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

        println!("real shutting down");


    }

    pub fn shutdown(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    async fn do_shutdown(&mut self) {
        info!("Stopping server...");
        // if self.running {
        //     warn!("Stopping server while it is still running!");
        //     return;
        // }

        println!("o");
        // self.world.send(ServerShutdownEvent);
        println!("e");

        info!("CrabCraft has stopped! Bye!")
    }
}
    