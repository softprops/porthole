# porthole

[![Build Status](https://travis-ci.org/softprops/porthole.svg?branch=master)](https://travis-ci.org/softprops/porthole)

> A tiny rust crate for resolving the next available network port

## docs

Find them [here](http://softprops.github.io/porthole)

## install

Add the following to your `Cargo.toml`

```toml
[dependencies]
porthole = "0.1.0"
```

# usage

```rust
use porthole;
fn main() {
  let port = porthole::open().unwrap();
  println!("next open port {:?}", port);
}
```

Doug Tangren (softprops) 2015
