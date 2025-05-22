use std::net::Ipv4Addr;
use cidr;

// fn test_1() {
//     let cidr = cidr::Ipv4Cidr::from_str("192.168.0.0/16").unwrap();
//     for addr in cidr {
//         println!("{}", addr);
//     }
// }
// 
// fn test_2() {
//     let cidr = cidr::Ipv4Inet::from_str("192.168.1.0/16")
//         .unwrap()
//         .network();
//     for addr in cidr {
//         println!("{}", addr);
//     }
// }

fn main() {
    // test_1();
    let ip = Ipv4Addr::new(192, 168, 0, 0);
    let cidr = cidr::Ipv4Cidr::new(ip, 16).unwrap();
    for _ in cidr {
        // println!("{}", addr);
    }
}
