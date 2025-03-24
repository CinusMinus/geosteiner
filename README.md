# geosteiner bindings

[![crates.io](https://img.shields.io/crates/l/geosteiner.svg?style=flat)](https://crates.io/crates/geosteiner)
[![crates.io](https://img.shields.io/crates/v/geosteiner.svg?style=flat)](https://crates.io/crates/geosteiner)
[![docs.rs](https://docs.rs/geosteiner/badge.svg)](https://docs.rs/geosteiner)

Compute Euclidean and rectilinear minimum Steiner trees using safe bindings to the [geosteiner](http://geosteiner.com) C library.

## Usage

The library provides two functions to compute the Euclidean and rectilinear minimum Steiner trees.

```rust
use geosteiner::euclidean_steiner_tree;

fn main() {
    let terms = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let tree = euclidean_steiner_tree(&terms);
    println!("found tree with length {}", tree.length);
    println!("steiner points: ");
    println!("{:.2?}", tree.steiner_points);
    println!("edges: ");
    println!("{:?}", tree.edges);
    println!("length: {:.2}", tree.length);
}
```

## Safety
This crate is just a wrapper around the geosteiner C library.
While care has been taken to ensure that all ffi calls are safe, the underlying C code may still contain bugs that could lead to memory unsafety.
We take no responsibility for the geosteiner C library and state that its authors are unaffiliated with this crate.

## License
Unfortunately, geosteiner itself is licensed under CC-BY-NC, so these bindings are limited to the same license.