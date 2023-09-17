use crate::port_finders::PortFinder;
use crate::port_finders::OsQueryPortFinder;

pub fn find_unused_port() -> Option<u16> {
  OsQueryPortFinder::new().find_port()
}
