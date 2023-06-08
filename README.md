# subnet-rs

Return all ip addresses of a subnet

# Example

```
use subnet;

fn main() {
    let ret = match subnet::ipv4_iter("192.168.1.0", 24) {
        Some(ret) => ret,
        None => panic!("get subnet failed"),
    };
    for r in ret {
        println!("{:?}", r);
    }
}
```
