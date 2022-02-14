mod connection;
pub use connection::Connection;

mod error;
pub use error::Error;

#[derive(Debug)]
pub struct Config {
    pub address: String,
    pub username: String,
    pub password: String,
    pub ssh_config: Option<String>,
}
