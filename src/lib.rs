
use std::net::TcpListener;
use std::io::Result;
use std::iter::Iterator;

/// Returns the next available network port
/// on the current host
pub fn next() -> Result<u16> {
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
    ::next().ok()
  }
}

pub fn iter() -> Iter {
  Iter
}

#[test]
fn it_works() {
}
