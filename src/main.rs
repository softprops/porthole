extern crate port;

fn main() {
  for p in port::iter().take(5) {
    println!("{:?}", p);
  }
}
