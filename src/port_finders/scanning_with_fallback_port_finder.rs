use super::ScanningPortFinder;
use super::PortFinder;
use super::OsQueryPortFinder;

pub struct ScanningWithFallbackPortFinder<const MIN: u16, const MAX: u16> {
  scanner: ScanningPortFinder<MIN, MAX>,
  random: OsQueryPortFinder,
}

impl<const MIN: u16, const MAX: u16> ScanningWithFallbackPortFinder<MIN, MAX> {
  pub const fn new() -> Self {
    Self {
      scanner: ScanningPortFinder::new(),
      random: OsQueryPortFinder::new(),
    }
  }
}

impl<const MIN: u16, const MAX: u16> PortFinder for ScanningWithFallbackPortFinder<MIN, MAX> {
  fn find_port(&mut self) -> Option<u16> {
      self.scanner.find_port().or_else(|| self.random.find_port())
  }
}
