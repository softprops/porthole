# porthole

> A tiny rust crate for resolving the next available network port


# usage

```rust
use porthole;
fn main() {
  let port = porthole::open().unwrap();
  println!("next open port {:?}", port);
}
```


Doug Tangren (softprops) 2015
