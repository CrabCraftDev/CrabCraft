use crab_protocol::error::ProtocolError;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    InvalidJson(#[from] serde_json::Error),
    #[error(transparent)]
    Protocol(#[from] ProtocolError),
}
