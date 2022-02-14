use log::{debug, info};
use remotefs::RemoteFs;
use remotefs_ssh::{SftpFs, SshOpts};
use std::{io::Write, path::Path};

use super::{error::*, Config};

mod host;
use host::parse_host;
pub struct Connection<C: RemoteFs> {
    client: C,
}

impl<C: RemoteFs> Connection<C> {
    /// Copy files to destination
    pub fn copy(&mut self, files: Vec<String>, destination: String) -> Result<()> {
        let remote_destination_path_string = Path::new(destination.as_str());

        for local_file_path_string in files {
            let local_file_path = Path::new(local_file_path_string.as_str());

            info!("Opening local file: {local_file_path:?}");
            let local_metadata = local_file_path.metadata()?;

            let remote_destination_path = {
                let filename = local_file_path
                    .file_name()
                    .ok_or_else(|| Error::FileNotFound(local_file_path.display().to_string()))?;
                remote_destination_path_string.join(filename)
            };

            info!("Creating remote file: {remote_destination_path:?}");
            let mut remote_file_handle = self
                .client
                .create(&remote_destination_path, &local_metadata.into())?;

            let file_contents = std::fs::read(local_file_path)?;
            info!("Copying: {local_file_path:?} to {remote_destination_path:?}");
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
        let destination = parse_host(config.address)?;

        debug!("Destination: {destination:?}");

        let client_builder = {
            let partial_client_builder = SshOpts::new(destination.hostname)
                .port(destination.port)
                .username(config.username)
                .password(config.password);

            if let Some(ssh_config) = config.ssh_config {
                info!("using config: {ssh_config}");
                partial_client_builder.config_file(ssh_config)
            } else {
                partial_client_builder
            }
        };

        let mut client: SftpFs = client_builder.into();
        client.connect()?;

        Ok(Connection { client })
    }
}
