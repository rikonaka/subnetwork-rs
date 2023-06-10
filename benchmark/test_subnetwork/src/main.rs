use subnetwork::Ipv4Pool;

fn main() {
    let ret = Ipv4Pool::new("192.168.1.0/24").unwrap();
    for _ in ret {
        // println!("{}", ip);
    }
}
