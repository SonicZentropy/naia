use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use regex::Regex;

use super::server_addr::ServerAddr;

pub fn candidate_to_addr(candidate_str: &str) -> ServerAddr {
    let pattern =
        Regex::new(r"\b(?P<ip_addr>(?:[0-9]{1,3}\.){3}[0-9]{1,3}) (?P<port>[0-9]{1,5})\b")
            .expect("failed to compile regex pattern");

    let captures = pattern
        .captures(candidate_str)
        .expect("regex failed to find SocketAddr string");

    let ip_addr = captures["ip_addr"]
        .parse::<Ipv4Addr>()
        .expect("not a valid ip address..");
    let port = &captures["port"].parse::<u16>().expect("not a valid port..");

    ServerAddr::Found(SocketAddr::new(IpAddr::V4(ip_addr), *port))
}

#[cfg(test)]
mod tests {

    use crate::{server_addr::ServerAddr, wasm_utils::candidate_to_addr};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[test]
    fn candidate_to_addr_works() {
        assert_eq!(
            candidate_to_addr("candidate:1 1 UDP 1755993416 127.0.0.1 14192 typ host"),
            ServerAddr::Found(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                14192
            ))
        );
    }
}