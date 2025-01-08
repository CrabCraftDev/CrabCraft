use evenio::event::GlobalEvent;
use crab_macros::AsyncEvent;
use crate::events::asynchronous::AsyncEvent;
use crate::utils::errors::Error;

#[derive(GlobalEvent, AsyncEvent, Debug)]
pub struct ServerShutdownEvent;
