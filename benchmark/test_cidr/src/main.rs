use cidr;

fn main() {
    let start = "192.168.1.1".parse().unwrap();
    let end = "192.168.1.254".parse().unwrap();
    let inet = cidr::Ipv4InetPair::new(start, end).unwrap();
    for i in inet.iter() {
        println!("{:?}", i);
    }
}
