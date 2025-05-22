use ipnetwork::IpNetwork;

fn main() {
    let net: IpNetwork = "192.168.0.0/16".parse().unwrap();
    for _ in net.iter() {
        // println!("{}", i);
    }
}
