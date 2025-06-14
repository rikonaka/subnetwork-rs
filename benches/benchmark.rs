use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use std::hint::black_box;

use cidr::Ipv4Cidr;
use ipnet::Ipv4Net;
use ipnetwork::IpNetwork;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use subnetwork::Ipv4Pool;

fn cidr_func(tests: usize) {
    let ipv4 = Ipv4Addr::new(192, 168, 0, 0);
    let cidr = Ipv4Cidr::new(ipv4, 16).unwrap();
    for _ in 0..tests {
        for _addr in cidr {
            // println!("{}", _addr);
            black_box(_addr);
        }
    }
}

fn ipnetwork_func(tests: usize) {
    let ipv4 = Ipv4Addr::new(192, 168, 0, 0);
    let ip = IpAddr::V4(ipv4);
    let net = IpNetwork::new(ip, 16).unwrap();
    for _ in 0..tests {
        for _addr in net.iter() {
            // println!("{}", _addr);
            black_box(_addr);
        }
    }
}

fn ipnet_func(tests: usize) {
    let ipv4 = Ipv4Addr::new(192, 168, 0, 0);
    let net = Ipv4Net::new(ipv4, 16).unwrap();
    for _ in 0..tests {
        for _addr in net.hosts() {
            // println!("{}", _addr);
            black_box(_addr);
        }
    }
}

fn subnetwork_func(tests: usize) {
    let ipv4 = Ipv4Addr::new(192, 168, 0, 0);
    let pool = Ipv4Pool::new(ipv4, 16).unwrap();
    for _ in 0..tests {
        for _addr in pool {
            // println!("{}", _addr);
            black_box(_addr);
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let tests = 100;
    c.bench_function("cidr", |b| b.iter(|| black_box(cidr_func(tests))));
    c.bench_function("ipnetwork", |b| b.iter(|| black_box(ipnetwork_func(tests))));
    c.bench_function("ipnet", |b| b.iter(|| black_box(ipnet_func(tests))));
    c.bench_function("subnetwork", |b| {
        b.iter(|| black_box(subnetwork_func(tests)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
