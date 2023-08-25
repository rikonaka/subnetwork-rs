# subnetwork

Returns an iterator that iterates over all subnet IPs.

[![Rust](https://github.com/rikonaka/subnetwork-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/rikonaka/subnetwork-rs/actions/workflows/rust.yml)

# Example

```rust
use subnetwork::{Ipv4Pool，Ipv4};

fn main() {
    let ipv4 = Ipv4::from("192.168.1.1").unwrap();
    let ipv4_pool = Ipv4Pool::new("192.168.1.0/24").unwrap();
    for i in ipv4.iter(24) {
        println!("{:?}", i);
    }
    for i in ipv4_pool {
        println!("{:?}", i);
    }
    let ret = ipv4_pool.contain_from_str("192.168.1.200").unwrap();
    assert_eq!(ret, true);
    let ret = ipv4_pool.contain(ipv4);
    assert_eq!(ret, true);

    let ret = ipv4.within_from_str("192.168.1.0/24").unwarp();
    assert_eq!(ret, true);
    let ret = ipv4.within(ipv4_pool);
    assert_eq!(ret, true);
}
```

# Benchmark

You can see how our performance compares to other similar libraries [here](./benchmark/README.md).
