use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum Error {
    InvalidUrl,
    TcpStreamConnectError,
    RequestError,
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidUrl => write!(f, "Invalid URL"),
            Error::TcpStreamConnectError => write!(f, "Tcp Stream Connect Error"),
            Error::RequestError => write!(f, "Request Error"),
        }
    }
}
