
use std::net::TcpListener;
use std::io::Result;
use std::iter::Iterator;

/// Returns the next available network port
/// on the current host
pub fn open() -> Result<u16> {
  TcpListener::bind("0.0.0.0:0")
    .and_then(|l| {
      let addr = l.local_addr();
      drop(l);
      addr
     })
    .and_then(|a| Ok(a.port()))
}

pub struct Iter;

impl Iterator for Iter {
  type Item = u16;
  fn next(&mut self) -> Option<u16> {
    ::open().ok()
  }
}

pub fn iter() -> Iter {
  Iter
}


#[test]
fn resolves_open_ports() {
  assert_eq!(open().is_ok(), true)
}

#[test]
fn provides_iter_over_open_ports() {
  assert_eq!(iter().take(5).count(), 5)
}
