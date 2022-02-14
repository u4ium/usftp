use crate::error::*;

#[derive(Debug, PartialEq)]
pub struct Host {
    pub hostname: String,
    pub port: u16,
}

pub fn parse_host(address: String) -> Result<Host> {
    if let Some((hostname, port)) = address.split_once(':') {
        Ok(Host {
            hostname: hostname.to_owned(),
            port: port.parse::<u16>()?,
        })
    } else {
        Ok(Host {
            hostname: address,
            port: 22,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_host, Host};

    mod parse_host {
        use super::*;

        #[test]
        fn should_default_to_port_22_if_no_colon() {
            let expected_host = Host {
                hostname: "test.domain.com".to_owned(),
                port: 22,
            };
            let actual_host = parse_host("test.domain.com".to_owned()).unwrap();
            assert_eq!(expected_host, actual_host);
        }

        #[test]
        fn should_parse_port_and_host_with_colon() {
            let expected_host = Host {
                hostname: "test.domain.com".to_owned(),
                port: 2222,
            };
            let actual_host = parse_host("test.domain.com:2222".to_owned()).unwrap();
            assert_eq!(expected_host, actual_host);
        }
    }
}
