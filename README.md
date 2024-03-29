# usftp

The Unsafe-Safe File Transfer Protocol Program

This program is only "Unsafe" in that is takes the password to connect in plaintext on the command line. Its is designed exactly for that purpose.

## Build and run

This is a basic rust CLI program, so it can be run from source using [cargo](https://doc.rust-lang.org/cargo/). The arguments to USFTP go after the `--` separator, as in: `cargo run -- YOUR_ARGS_HERE`.

Cargo can also compile a release binary with the command `cargo build --release`, after which a file called `usftp.exe` is created under `target/release/`, which is specific to your platform. Then, this binary is run with `usftp.exe YOUR_ARGS_HERE`.

### Prebuilt Binaries

A recent release of `usftp.exe` for Windows should be available [here](https://github.com/u4ium/usftp/releases/latest).

### Examples

- Get help about all the command-line arguments:
  ```console
  cargo run -- --help
  ```
  OR
  ```console
  usftp.exe --help
  ```
- Copy a file called `test.txt` into the `/tmp` folder on the `remote.server.com`:
  ```console
  usftp.exe --address remote.server.com --user admin --password pass /tmp test.txt
  ```
- Do the same thing with shorter argument names:
  ```console
  usftp.exe -a remote.server.com -u admin -p pass /tmp test.txt
  ```
- Use your own `.ssh/config` file (called `my_ssh_config`) and a non-standard port (`222`):
  ```console
  usftp.exe -c my_ssh_config -a remote.server.com:222 -u admin -p pass /tmp test.txt
  ```

### Usage

```console
USAGE:
    usftp.exe [OPTIONS] --address <ADDRESS> --user <USER> --password <PASSWORD> <DESTINATION> <FILES>...

ARGS:
    <DESTINATION>    The destination path (on the remote host) to copy the files to
    <FILES>...       One mandatory file followed by several optional files

OPTIONS:
    -a, --address <ADDRESS>      The URI address for the host: format <HOSTNAME:PORT> OR <HOSTNAME>
                                 (port is 22 by default)
    -c, --config <CONFIG>        An (optional) path to an SSH config file (defaults to ~/.ssh/config)
    -h, --help                   Print help information
    -p, --password <PASSWORD>    The password used to log onto the server
    -u, --user <USER>            The username used to log onto the server
    -V, --version                Print version information
```
