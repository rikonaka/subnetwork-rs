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
    // pool is copied
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

## Extended Ipv4Addr

```rust
use std::net::Ipv4Addr;
use subnetwork::Ipv4AddrExt;

fn main() {
    let ip1 = Ipv4Addr::new(192, 168, 1, 0);
    let ip2 = Ipv4Addr::new(192, 168, 1, 255);
    let ip1ext: Ipv4AddrExt = ip1.into();
    assert_eq!(ip1ext.largest_identical_prefix(ip2), 24);

    let ip1 = Ipv4Addr::new(192, 168, 1, 136);
    let ip2 = Ipv4Addr::new(192, 168, 1, 192);
    let ip1ext: Ipv4AddrExt = ip1.into();
    assert_eq!(ip1ext.largest_identical_prefix(ip2), 25);
}
```

## Extended Ipv6Addr

```rust
use std::net::Ipv6Addr;
use subnetwork::Ipv6AddrExt;

fn main() {
    let ipv6 = Ipv6Addr::from_str("::ffff:192.10.2.255").unwrap();
    let ipv6_ext: Ipv6AddrExt = ipv6.into();

    let ipv6_node_multicast = Ipv6Addr::from_str("ff01::1:ff0a:2ff").unwrap();
    assert_eq!(ipv6_ext.node_multicast(), ipv6_node_multicast);

    let ipv6_link_multicast = Ipv6Addr::from_str("ff02::1:ff0a:2ff").unwrap();
    assert_eq!(ipv6_ext.link_multicast(), ipv6_link_multicast);

    let ipv6_site_multicast = Ipv6Addr::from_str("ff05::1:ff0a:2ff").unwrap();
    assert_eq!(ipv6_ext.site_multicast(), ipv6_site_multicast);
}
```

## Extended Netmask

```rust
use std::net::Ipv4Addr;
use subnetwork::NetmaskExt;

fn main() {
    let netmask = NetmaskExt::new(24);
    let netmask_addr = netmask.to_ipv4().unwrap();
    assert_eq!(netmask_addr, Ipv4Addr::new(255, 255, 255, 0));

    let netmask = NetmaskExt::new(26);
    let netmask_addr = netmask.to_ipv4().unwrap();
    assert_eq!(netmask_addr, Ipv4Addr::new(255, 255, 255, 192));
}
```

## Benchmark

You can see how our performance compares to other similar libraries [here](./benches/README.md).
