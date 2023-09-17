use super::PortFinder;
use crate::is_port_available;

const RETRY_FOUND_MIN : u32 = 500;

/// A `PortFinder` which will scan for ports along a known range.
pub struct ScanningPortFinder<const MIN: u16, const MAX: u16> {
  last: u16,
  found_count: u32,
}

impl<const MIN: u16, const MAX: u16> ScanningPortFinder<MIN, MAX> {
  pub const fn new() -> Self {
      Self {
        last: MIN,
        found_count: 0,
      }
  }
}

impl<const MIN: u16, const MAX: u16> PortFinder for ScanningPortFinder<MIN, MAX> {
    fn find_port(&mut self) -> Option<u16> {
        // This is hit if we loop round the port list,
        // and come back to the start.
        if self.last >= MAX {
          // We found very few ports,
          // then don't bother wrapping,
          if self.found_count < RETRY_FOUND_MIN {
            return None;
          }

          // Otherwise reset the min and wrap around.
          // We will probably find the ports we used last time.
          *self = Self::new();
        }

        let maybe_found = (self.last..MAX).find(|port| is_port_available(*port));
        if let Some(port) = maybe_found {
          self.last = port + 1; // Set 1 last the port,
          self.found_count += 1;
        }

        maybe_found
    }
}

#[cfg(test)]
mod tests_find_port {
    use super::*;

    #[test]
    fn it_finds_a_random_port() {
        assert!(ScanningPortFinder::<8000, 9999>::new().find_port().is_some());
    }
}
