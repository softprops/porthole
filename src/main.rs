extern crate porthole;

fn main() {
  for p in porthole::iter().take(5) {
    println!("{:?}", p);
  }
}
