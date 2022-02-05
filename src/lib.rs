use remotefs::{RemoteError, RemoteFs};
use remotefs_ftp::FtpFs;
use std::path::Path;
use thiserror::Error;

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
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub username: String,
    pub password: String,
}

pub struct Connection<C: RemoteFs> {
    client: C,
}

impl<C: RemoteFs> Connection<C> {
    pub fn copy(&mut self, files: Vec<String>, destination: String) -> Result<()> {
        // TODO: REMOVE TEST CODE

        // get working directory
        println!(
            "Working Directory: {}",
            self.client.pwd().ok().unwrap().display()
        );
        // change working directory
        assert!(self.client.change_dir(Path::new("/tmp")).is_ok());
        // disconnect
        assert!(self.client.disconnect().is_ok());
        Ok(())
    }
}

impl Connection<FtpFs> {
    pub fn new(config: Config) -> Result<Self> {
        let destination = parse_host(config.host)?;

        let mut client = FtpFs::new(destination.hostname, destination.port)
            .secure(true, true)
            .username(config.username)
            .password(config.password);

        client.connect()?;

        // TODO: go to client_path or store for later
        Ok(Connection { client })
    }
}

#[derive(Debug)]
struct Host {
    hostname: String,
    port: u16,
}

fn parse_host(host: String) -> Result<Host> {
    let (hostname, port) = host
        .split_once(':')
        .ok_or_else(|| Error::BadDestination(host.clone()))?;

    Ok(Host {
        hostname: hostname.to_owned(),
        port: port.parse::<u16>()?,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
