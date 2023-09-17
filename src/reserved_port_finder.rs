use crate::PortFinder;
use crate::ScanningWithFallbackPortFinder;

use std::collections::HashSet;

pub struct ReservedPortFinder<const MIN: u16, const MAX: u16> {
    finder: ScanningWithFallbackPortFinder<MIN, MAX>,
    ports_in_user: HashSet<u16>,
}

impl<const MIN: u16, const MAX: u16> ReservedPortFinder<MIN, MAX> {
    pub fn new() -> Self {
        Self {
            finder: ScanningWithFallbackPortFinder::new(),
            ports_in_user: HashSet::new(),
        }
    }

    /// Sets a port to be _permanently_ reserved.
    ///
    /// Reservations only affect this port scanner.
    pub fn reserve_port(&mut self, port: u16) {
      self.ports_in_user.insert(port);
    }

    #[must_use]
    pub fn reserve_random_port(&mut self) -> Option<u16> {
        // As long as the port finder can keep finding ports,
        // we keep spinning, and checking if the port is in use.
        //
        // When a free port is found, we return it.
        while let Some(port) = self.finder.find_port() {
            if self.ports_in_user.contains(&port) {
                continue;
            }

            self.ports_in_user.insert(port);
            return Some(port);
        }

        None
    }

    pub fn free_port(&mut self, port: u16) {
      self.ports_in_user.remove(&port);
    }
}
