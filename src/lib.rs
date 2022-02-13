use log::{debug, info};
use remotefs::{RemoteError, RemoteFs};
use remotefs_ssh::{SftpFs, SshOpts};
use std::{io::Write, path::Path};
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
    /// Copy files to destination
    pub fn copy(&mut self, files: Vec<String>, destination: String) -> Result<()> {
        let destination = Path::new(&destination[..]);
        for filepath_string in files {
            let filepath = Path::new(&filepath_string[..]);
            let filename = filepath.file_name().expect("bad filename");
            let remote_destination = destination.join(filename);
            let remote_metadata = filepath.metadata()?;
            info!("Copying: {filepath:?} to {remote_destination:?}");
            let mut remote_file_handle = self
                .client
                .create(&remote_destination, &remote_metadata.into())?;

            let file_contents = std::fs::read(filepath)?;
            remote_file_handle.write(&file_contents)?;
        }

        Ok(())
    }

    pub fn disconnect(&mut self) -> Result<()> {
        self.client.disconnect()?;
        Ok(())
    }
}

impl Connection<SftpFs> {
    pub fn new(config: Config) -> Result<Self> {
        let destination = parse_host(config.host)?;

        debug!("Destination: {destination:?}");

        let mut client: SftpFs = SshOpts::new(destination.hostname)
            .port(destination.port)
            .username(config.username)
            .password(config.password)
            .into();

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
