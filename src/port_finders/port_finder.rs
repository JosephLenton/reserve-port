pub trait PortFinder {
  fn find_port(&mut self) -> Option<u16>;
}
