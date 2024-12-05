use crate::events::asynchronous::AsyncEvent;
use crate::utils::errors::Error;
use crab_macros::AsyncEvent;
use evenio::event::GlobalEvent;

#[derive(GlobalEvent, AsyncEvent, Debug)]
pub struct WorldTickEvent;
