use ipnetwork::IpNetwork;

fn main() {
    let net: IpNetwork = "192.168.1.0/16".parse().unwrap();
    for _ in net.iter() {
        // println!("{}", i);
    }
}
