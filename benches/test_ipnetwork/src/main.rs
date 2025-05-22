use ipnetwork::IpNetwork;
use std::hint::black_box;
use std::net::IpAddr;
use std::net::Ipv4Addr;

fn my_test() {
    let ipv4 = Ipv4Addr::new(192, 168, 0, 0);
    let ip = IpAddr::V4(ipv4);
    let net = IpNetwork::new(ip, 16).unwrap();
    for _addr in net.iter() {
        // println!("{}", _addr);
    }
}

fn main() {
    black_box(my_test());
}
