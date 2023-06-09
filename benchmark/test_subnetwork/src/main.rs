use subnetwork::ipv4_iter;

fn main() {
    let ret = ipv4_iter("192.168.1.0/16").unwrap();
    for _ in ret {
        // println!("{}", ip);
    }
}
