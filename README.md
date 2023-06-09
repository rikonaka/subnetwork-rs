# subnetwork

Returns an iterator that iterates over all subnet IPs.

# Example

```rust
use subnetwork;

fn main() {
    let ret = subnetwork::ipv4_within_subnet("192.168.1.0/24", "192.168.1.200");
    println!("{:?}", ret);

    let ips = match subnetwork::ipv4_iter("192.168.1.0/24") {
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
true
192.168.1.1
192.168.1.2
192.168.1.3
192.168.1.4
192.168.1.5
192.168.1.6
192.168.1.7
192.168.1.8
192.168.1.9
...
192.168.1.255
```
