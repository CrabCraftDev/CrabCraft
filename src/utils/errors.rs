use crate::GITHUB_ISSUES_URL;
use crab_config::error::{ConfigError, DetailedConfigError};
use crab_macros::DetailedFrom;
use crab_nbt::error::Error as CrabNbtError;
use crab_protocol::error::{DetailedProtocolError, ProtocolError};
use crab_registry::error::{DetailedRegistryError, RegistryError};
use crab_world::error::{AnvilError, DetailedAnvilError, DetailedWorldError, WorldError};
use kanal::{OneshotReceiveError, SendError};
use std::{any::Any, io};
use thiserror::Error;
use tracing::error;

#[derive(Error, DetailedFrom, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    InvalidJson(#[from] serde_json::Error),
    #[error(transparent)]
    Protocol(
        #[from]
        #[detailed(DetailedProtocolError)]
        ProtocolError,
    ),
    #[error(transparent)]
    InvalidNbt(#[from] CrabNbtError),
    #[error(transparent)]
    World(
        #[from]
        #[detailed(DetailedWorldError)]
        WorldError,
    ),
    #[error(transparent)]
    Anvil(
        #[from]
        #[detailed(DetailedAnvilError)]
        AnvilError,
    ),
    #[error(transparent)]
    Config(
        #[from]
        #[detailed(DetailedConfigError)]
        ConfigError,
    ),
    #[error(transparent)]
    Registry(
        #[from]
        #[detailed(DetailedRegistryError)]
        RegistryError,
    ),

    #[error(transparent)]
    SendChannel(#[from] SendError),
    #[error(transparent)]
    OneshotReceive(#[from] OneshotReceiveError),

    #[error("Invalid async event type")]
    InvalidAsyncEventType,
}

pub fn print_panic(panic: Box<dyn Any + Send + 'static>) {
    if let Some(panic_string) = panic.downcast_ref::<String>() {
        error!("Panic error: {panic_string}");
    } else if let Some(panic_string) = panic.downcast_ref::<&'static str>() {
        error!("Panic error: {panic_string}");
    } else {
        error!("Panic error: is not a string and can't be printed");
    }
}

pub fn print_error_report_instructions() {
    error!("");
    error!("This error should not normally happen!");
    error!("Please report it at:");
    error!("{GITHUB_ISSUES_URL}");
    error!("");
}
