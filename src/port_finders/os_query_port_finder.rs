use std::net::{
    Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6,
};

use super::PortFinder;
use crate::is_port_available_udp;
use crate::bind_and_get_port_tcp;

const NUM_TRIES : usize = 100;

/// A `PortFinder` which will ask the OS for the port.
pub struct OsQueryPortFinder {}

impl OsQueryPortFinder {
    pub const fn new() -> Self {
        Self {}
    }
}

impl PortFinder for OsQueryPortFinder {
    fn find_port(&mut self) -> Option<u16> {
        (0..NUM_TRIES).find_map(|_| {
            ask_free_tcp_port()
                .filter(|port| is_port_available_udp(*port))
        })
    }
}

/// Ask the OS for a TCP port that is available.
fn ask_free_tcp_port() -> Option<u16> {
  let ipv4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
  let ipv6 = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0);

  bind_and_get_port_tcp(ipv6).or_else(|| bind_and_get_port_tcp(ipv4))
}

#[cfg(test)]
mod tests_find_port {
    use super::*;

    #[test]
    fn it_finds_a_random_port() {
        assert!(OsQueryPortFinder::new().find_port().is_some());
    }
}
