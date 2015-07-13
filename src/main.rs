extern crate porthole;

pub fn main() {
  match porthole::open() {
    Ok(port) => println!("{}",port),
    Err(e)   => panic!(format!("unable to resolve an open port: {}", e))
  }
}
