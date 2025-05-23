# subnetwork

Returns an iterator that iterates over all subnet IPs.

[![Rust](https://github.com/rikonaka/subnetwork-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/rikonaka/subnetwork-rs/actions/workflows/rust.yml)

## Standard Example

```rust
use std::net::Ipv4Addr;
use subnetwork::Ipv4Pool;
use std::str::FromStr;

fn main() {
    let pool = Ipv4Pool::from_str("192.168.1.0/24").unwrap();
    // or
    // let pool: Ipv4Pool = "192.168.1.0/24".parse().unwrap();
    for i in pool {
        println!("{:?}", i);
    }
    let ipv4 = Ipv4Addr::new(192, 168, 1, 1);
    assert_eq!(pool.contain(ipv4);, true);
}
```

## Special Example

```rust
use std::net::Ipv4Addr;
use subnetwork::CrossIpv4Pool;

fn main() {
    let start = Ipv4Addr::new(192, 168, 1, 1);
    let end = Ipv4Addr::new(192, 168, 3, 254);
    let ips = CrossIpv4Pool::new(start, end).unwrap();
    for i in ips {
        println!("{:?}", i);
    }
}
```

## Benchmark

You can see how our performance compares to other similar libraries [here](./benches/README.md).
