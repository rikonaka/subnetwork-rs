# subnetwork

Returns an iterator that iterates over all subnet IPs.

[![Rust](https://github.com/rikonaka/subnetwork-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/rikonaka/subnetwork-rs/actions/workflows/rust.yml)

## Standard Example

```rust
use std::net::Ipv4Addr;
use std::str::FromStr;
use subnetwork::Ipv4Pool;

fn main() {
    let pool = Ipv4Pool::new(Ipv4Addr::new(192, 168, 1, 1), 24).unwrap();
    // from 192.168.1.0 to 192.168.1.255
    for ipv4 in pool {
        println!("{}", ipv4);
    }

    let pool = Ipv4Pool::from_str("192.168.1.0/24").unwrap();
    for ipv4 in pool {
        println!("{}", ipv4);
    }

    let pool: Ipv4Pool = "192.168.1.0/24".parse().unwrap();
    for ipv4 in pool {
        println!("{}", ipv4);
    }

    let test_ipv4 = Ipv4Addr::new(192, 168, 1, 233);
    assert_eq!(pool.contain(test_ipv4), true);

    let broadcast = Ipv4Addr::new(192, 168, 1, 255);
    assert_eq!(pool.broadcast(), broadcast);

    let network = Ipv4Addr::new(192, 168, 1, 0);
    assert_eq!(pool.network(), network);

    assert_eq!(pool.len(), 256);
    // pool is copied.
    assert_eq!(pool.to_string(), "192.168.1.0/24, next 192.168.1.0");
}
```

## Cross Multi Subnet

```rust
use std::net::Ipv4Addr;
use subnetwork::CrossIpv4Pool;

fn main() {
    let start = Ipv4Addr::new(192, 168, 1, 16);
    let end = Ipv4Addr::new(192, 168, 3, 200);
    let pool = CrossIpv4Pool::new(start, end).unwrap();
    // include 192.168.1.16 and 192.168.3.200
    for i in pool {
        println!("{:?}", i);
    }

    let test_ipv4 = Ipv4Addr::new(192, 168, 1, 233);
    assert_eq!(pool.contain(test_ipv4), true);
    let test_ipv4 = Ipv4Addr::new(192, 168, 2, 0);
    assert_eq!(pool.contain(test_ipv4), true);
    let test_ipv4 = Ipv4Addr::new(192, 168, 3, 255);
    assert_eq!(pool.contain(test_ipv4), false);
    let test_ipv4 = Ipv4Addr::new(192, 168, 3, 200);
    assert_eq!(pool.contain(test_ipv4), true);
}
```

## Ipv4AddrExt

```rust
use std::net::Ipv4Addr;
use subnetwork::Ipv4AddrExt;

fn main() {
    let ip1 = Ipv4Addr::new(192, 168, 1, 136);
    let ip2 = Ipv4Addr::new(192, 168, 1, 192);

    let ip1ext: Ipv4AddrExt = ip1.into();
    let ip2ext: Ipv4AddrExt = ip2.into();
    assert_eq!(ip1ext.largest_identical_prefix(ip2ext), 25);

    let ip1ext: Ipv4AddrExt = ip1.into();
    assert_eq!(ip1ext.largest_identical_prefix(ip2), 25);
}
```

## Benchmark

You can see how our performance compares to other similar libraries [here](./benches/README.md).
