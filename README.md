# subnetwork-rs

Return all ip addresses of a subnetwork.

# Example

```
use subnetwork;

fn main() {
    let ret = match Subnetwork::ipv4_iter("192.168.1.0", 24) {
        Some(ret) => ret,
        None => panic!("get subnet failed"),
    };
    for r in ret {
        println!("{:?}", r);
    }
}
```
