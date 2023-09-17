use ::std::net::{
  Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6, TcpListener, ToSocketAddrs, UdpSocket,
};

/// Check if a port is available on both TCP and UDP.
pub fn is_port_available(port: u16) -> bool {
  is_port_available_tcp(port) && is_port_available_udp(port)
}

/// Check if a port is available on TCP.
pub fn is_port_available_tcp(port: u16) -> bool {
  let ipv4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
  let ipv6 = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, port, 0, 0);

  bind_and_get_port_tcp(ipv6).is_some() && bind_and_get_port_tcp(ipv4).is_some()
}

/// Check if a port is available on UDP.
pub fn is_port_available_udp(port: u16) -> bool {
  let ipv4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
  let ipv6 = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, port, 0, 0);

  bind_and_get_port_udp(ipv6).is_some() && bind_and_get_port_udp(ipv4).is_some()
}

// Binds to a socket using UDP, and returns the Port in use.
pub(crate) fn bind_and_get_port_udp<A: ToSocketAddrs>(addr: A) -> Option<u16> {
  Some(UdpSocket::bind(addr).ok()?.local_addr().ok()?.port())
}

// Binds to a socket using TCP, and returns the Port in use.
pub(crate) fn bind_and_get_port_tcp<A: ToSocketAddrs>(addr: A) -> Option<u16> {
  Some(TcpListener::bind(addr).ok()?.local_addr().ok()?.port())
}
