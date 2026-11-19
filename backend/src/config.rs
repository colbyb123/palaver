use std::{
    ffi::OsStr,
    net::{Ipv4Addr, SocketAddrV4},
};

const DEFAULT_PORT: u16 = 8080;
const PORT_ERROR: &str = "PALAVER_PORT must be a decimal port from 1 to 65535";

pub fn listen_address(port: Option<&OsStr>) -> Result<SocketAddrV4, &'static str> {
    let port = match port {
        None => DEFAULT_PORT,
        Some(value) => {
            let value = value.to_str().ok_or(PORT_ERROR)?;
            if !value.bytes().all(|byte| byte.is_ascii_digit()) {
                return Err(PORT_ERROR);
            }
            value
                .parse::<u16>()
                .ok()
                .filter(|&port| port != 0)
                .ok_or(PORT_ERROR)?
        }
    };
    Ok(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_loopback_port_8080() {
        assert_eq!(listen_address(None), Ok("127.0.0.1:8080".parse().unwrap()));
    }

    #[test]
    fn accepts_an_override_and_keeps_loopback() {
        assert_eq!(
            listen_address(Some(OsStr::new("65535"))),
            Ok("127.0.0.1:65535".parse().unwrap())
        );
    }

    #[test]
    fn rejects_invalid_ports() {
        for value in ["", "0", "65536", "-1", "+80", " 80", "http"] {
            assert_eq!(listen_address(Some(OsStr::new(value))), Err(PORT_ERROR));
        }
    }
}
