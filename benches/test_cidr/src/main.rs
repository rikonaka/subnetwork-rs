use cidr;
use std::hint::black_box;
use std::net::Ipv4Addr;

fn my_test() {
    let ip = Ipv4Addr::new(192, 168, 0, 0);
    let cidr = cidr::Ipv4Cidr::new(ip, 16).unwrap();
    for addr in cidr {
        // println!("{}", addr);
        black_box(addr);
    }
}

fn main() {
    black_box(my_test());
}
