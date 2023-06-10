# subnetwork

Returns an iterator that iterates over all subnet IPs.

# Example

## 1

```rust
use subnetwork:Ipv4Pool;

fn ipv4_pool() {
    let ips = Ipv4Pool::new("192.168.1.1/24").unwrap();
    for i in ips {
        println!("{:?}", i);
    }
    let ret = ips.contain("192.168.1.200").unwrap();
    println!("{:?}", ret);
}
```

## 2

```rust
use subnetwork::Ipv4;

fn main() {
    let ip = Ipv4::new("192.168.1.1").unwrap();
    for i in ip.iter(24) {
        println!("{:?}", i);
    }
    let ret = ip.within("192.168.1.0/24").unwarp();
    println!("{:?}", ret);
}
```
## Output

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
...
192.168.1.255
true
```
