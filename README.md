# subnetwork

Returns an iterator that iterates over all subnet IPs.

# Example

```rust
use subnetwork;

fn main() {
    let ret = subnetwork::ipv4_within_subnet("192.168.1.0/24", "192.168.1.200");
    println!("{:?}", ret); // true

    let ips = match subnetwork::ipv4_iter("192.168.1.0", 24) {
        Some(ips) => ips,
        None => panic!("get subnet failed"),
    };
    for ip in ips {
        println!("{:?}", ip);
    }
}
```

**Output**

```bash
192.168.1.1
192.168.1.2
192.168.1.3
192.168.1.4
192.168.1.5
192.168.1.6
192.168.1.7
192.168.1.8
192.168.1.9
192.168.1.10
192.168.1.11
192.168.1.12
192.168.1.13
192.168.1.14
192.168.1.15
192.168.1.16
...
192.168.1.255
```
