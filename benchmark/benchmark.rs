use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;

use cidr::Ipv4Cidr;
use ipnetwork::IpNetwork;
use std::net::Ipv4Addr;
use subnetwork::Ipv4Pool;

fn cidr_func() {
    let ip = Ipv4Addr::new(192, 168, 0, 0);
    let cidr = Ipv4Cidr::new(ip, 16).unwrap();

    for _ in 0..1000 {
        for _ in cidr {
            // println!("{}", addr);
        }
    }
}

fn ipnetwork_func() {
    let net = IpNetwork::new(Ipv4Addr::new(192, 168, 0, 0), 16).unwrap();
    for _ in 0..1000 {
        for _ in net.hosts() {
            // println!("{}", addr);
        }
    }
}

fn main() {
    let pool = Ipv4Pool::new(Ipv4Addr::new(192, 168, 0, 0), 16).unwrap();
    for _ in 0..1000 {
        for _ in pool {
            // println!("{}", addr);
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("cidr", |b| b.iter(|| cidr_func()));
    c.bench_function("ipnetwork", |b| b.iter(|| ipnetwork_func()));
    c.bench_function("subnetwork", |b| b.iter(|| subnetwork_func()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
