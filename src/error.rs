use thiserror::Error;

use remotefs::RemoteError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("port error")]
    PortError(#[from] std::num::ParseIntError),
    #[error("Destination could not be parsed")]
    BadDestination(String),
    #[error("connection error")]
    BadConnection(#[from] RemoteError),
    #[error("file not found error")]
    FileNotFound(String),
}

pub type Result<T> = std::result::Result<T, Error>;
