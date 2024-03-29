use std::io;

extern crate env_logger;

#[macro_use]
extern crate clap;

use clap::Arg;

use usftp::{Config, Connection, Error};

fn get_command_line_args() -> io::Result<(Config, Vec<String>, String)> {
    let matches = app_from_crate!()
        .arg(
            Arg::new("address")
                .short('a')
                .long("address")
                .value_name("ADDRESS")
                .help("The URI address for the host: format <HOSTNAME:PORT> OR <HOSTNAME> (port is 22 by default)")
                .required(true),
        )
        .arg(
            Arg::new("user")
                .short('u')
                .long("user")
                .value_name("USER")
                .help("The username used to log onto the server")
                .required(true),
        )
        .arg(
            Arg::new("password")
                .short('p')
                .long("password")
                .value_name("PASSWORD")
                .help("The password used to log onto the server")
                .required(true),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("CONFIG")
                .help("An (optional) path to an SSH config file (defaults to ~/.ssh/config)")
                .required(false),
        )
        .arg(
            Arg::new("destination")
                .value_name("DESTINATION")
                .help("The destination path (on the remote host) to copy the files to")
                .required(true),
        )
        .arg(
            Arg::new("files")
                .value_names(&["FILES"])
                .help("One mandatory file followed by several optional files")
                .required(true)
                .min_values(1)
                .multiple_values(true),
        )
        .get_matches();

    let config = {
        let address = matches
            .value_of("address")
            .expect("named argument <address> should exist")
            .to_owned();
        let username = matches
            .value_of("user")
            .expect("named argument <user> should exist")
            .to_owned();
        let password = matches
            .value_of("password")
            .expect("named argument <password> should exist")
            .to_owned();
        let ssh_config = matches.value_of("config").map(str::to_owned);

        Config {
            address,
            username,
            password,
            ssh_config,
        }
    };

    let files = matches
        .values_of("files")
        .expect("positional argument(s) <files> should exist")
        .map(str::to_owned)
        .collect();

    let destination = matches
        .value_of("destination")
        .expect("positional argument <destination> should exist")
        .to_owned();

    Ok((config, files, destination))
}

fn main() -> Result<(), Error> {
    #[cfg(debug_assertions)]
    env_logger::init();

    let (config, files, destination) = get_command_line_args()?;
    let mut connection = Connection::new(config)?;
    connection.copy(files, destination)?;
    connection.disconnect()?;
    Ok(())
}
