use subnetwork::Ipv4;

fn main() {
    let ret = Ipv4::new("192.168.1.1").unwrap();
    for _ in ret.iter(24) {
        // println!("{}", ip);
    }
}
