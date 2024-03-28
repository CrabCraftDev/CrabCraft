use crab_protocol::error::ProtocolError;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IOError(#[from] io::Error),
    #[error(transparent)]
    ProtocolError(#[from] ProtocolError),
}
