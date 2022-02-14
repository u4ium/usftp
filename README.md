# usftp
The Unsafe-Safe File Transfer Protocol Program

## Build and run

### Build And Run
```bash
cargo run -- --help
```

### Run

```bash
USAGE:
    usftp.exe --host <HOST> --user <USER> --password <PASSWORD> <DESTINATION> <FILES>...

ARGS:
    <DESTINATION>    The destination path (on the host) to copy the files to
    <FILES>...       One mandatory file followed by several optional files

OPTIONS:
    -h, --host <HOST>            The URI for the host: format <HOSTNAME:PORT>
        --help                   Print help information
    -p, --password <PASSWORD>    The password used to log onto the server
    -u, --user <USER>            The username used to log onto the server
    -V, --version                Print version information
```
