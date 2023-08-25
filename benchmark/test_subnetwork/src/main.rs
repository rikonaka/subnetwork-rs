use subnetwork::Ipv4Pool;

fn main() {
    let ret = Ipv4Pool::from("192.168.0.0/24").unwrap();
    for _ in ret {
        // println!("{}", ip);
    }
}
