use ::std::sync::Mutex;
use ::lazy_static::lazy_static;

use crate::Error;
use crate::Result;
use crate::ReservedPortFinder;

const MIN_PORT: u16 = 8_000;
const MAX_PORT: u16 = 9_999;

lazy_static! {
    static ref GLOBAL_PORT_FINDER: Mutex<ReservedPortFinder<MIN_PORT, MAX_PORT>> = Mutex::new(ReservedPortFinder::new());
}

/// A port, that at the time of creation, is guaranteed to be free for use by the OS.
/// This also guarantees not to clash with _other_ `ReservedPort` objects.
///
/// The motivation of this library is to allow one to reserve many ports,
/// ensure they don't clash with each other,
/// and then let them go when they are no longer needed.
#[derive(Debug)]
pub struct ReservedPort {
    port: u16,
}

impl ReservedPort {
    fn new(port: u16) -> Self {
        Self {
            port,
        }
    }

    pub fn random() -> Result<Self> {
        let mut port_finder = GLOBAL_PORT_FINDER
            .lock()
            .map_err(|_| Error::InternalLockError)?;

        port_finder.reserve_random_port()
            .map(ReservedPort::new)
            .ok_or(Error::FailedToReservePort)
    }

    /// _Permanently_ reserves the given port as being offlimits (for this library).
    ///
    /// This is useful if you have connected to a socket yourself,
    /// and wish to avoid clashing with this library.
    #[must_use]
    pub fn reserve_port(port: u16) -> Result<()> {
        let mut port_finder = GLOBAL_PORT_FINDER
            .lock()
            .map_err(|_| Error::InternalLockError)?;

        port_finder.reserve_port(port);

        Ok(())
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

impl Drop for ReservedPort {
    fn drop(&mut self) {
        let mut port_finder = GLOBAL_PORT_FINDER
            .lock()
            .map_err(|_| Error::InternalLockError)
            .expect("Should be able to unlock global port finder");

        port_finder
            .free_port(self.port);
    }
}

#[cfg(test)]
mod test_reserve_port {
    use super::*;

    #[test]
    fn it_should_reserve_a_port_for_use() {
        const TEST_PORT_NUM: u16 = 1230;

        let reserved = ReservedPort::reserve_port(TEST_PORT_NUM);

        assert!(reserved.is_ok());
    }

    #[test]
    fn it_should_reserve_same_port_twice_in_a_row() {
        const TEST_PORT_NUM: u16 = 1231;

        let _ = ReservedPort::reserve_port(TEST_PORT_NUM);
        let reserved = ReservedPort::reserve_port(TEST_PORT_NUM);

        assert!(reserved.is_ok());
    }

    #[test]
    fn it_should_allow_reserving_random_ports_by_hand() {
        let reserved_1 = ReservedPort::random().unwrap();
        let reserved_2 = ReservedPort::reserve_port(reserved_1.port());

        assert!(reserved_2.is_ok());
    }

    #[test]
    fn it_should_allow_reserving_random_ports_by_hand_after_they_have_dropped() {
        let reserved_1 = ReservedPort::random().unwrap();
        let random_port = reserved_1.port();
        ::std::mem::drop(reserved_1);

        let result = ReservedPort::reserve_port(random_port);

        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod test_reserve_random_port {
    use super::*;

    #[test]
    fn it_should_reserve_a_random_port_for_use() {
        let reserved = ReservedPort::random();

        assert!(reserved.is_ok());
    }

    #[test]
    fn it_should_reserve_different_ports_over_use() {
        let reserved_1 = ReservedPort::random().unwrap();
        let reserved_2 = ReservedPort::random().unwrap();

        assert_ne!(reserved_1.port(), reserved_2.port());
    }
}
