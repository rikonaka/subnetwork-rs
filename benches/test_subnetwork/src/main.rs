use std::hint::black_box;
use std::net::Ipv4Addr;
use subnetwork::Ipv4Pool;

fn my_test() {
    let ipv4 = Ipv4Addr::new(192, 168, 0, 0);
    let pool = Ipv4Pool::new(ipv4, 16).unwrap();
    for _addr in pool {
        // println!("{}", _addr);
    }
}

fn main() {
    black_box(my_test());
}
