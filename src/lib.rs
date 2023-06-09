use std::net::{Ipv4Addr, Ipv6Addr};

const NEXT_VALUE: usize = 1;
const IPV4_LEN: usize = 32;
const IPV6_LEN: usize = 128;

#[derive(Debug)]
pub struct Ipv4PoolRaw {
    address_b: u32,
    prefix_b: u32,
    next: u32,
    stop: u32,
}

#[derive(Debug)]
pub struct Ipv6PoolRaw {
    address_b: u128,
    prefix_b: u128,
    next: u128,
    stop: u128,
}

#[derive(Debug)]
pub struct Ipv4Pool {
    address_b: u32,
    prefix_b: u32,
    next: u32,
    stop: u32,
}

#[derive(Debug)]
pub struct Ipv6Pool {
    address_b: u128,
    prefix_b: u128,
    next: u128,
    stop: u128,
}

#[derive(Debug)]
pub struct Ipv4PoolString {
    address_b: u32,
    prefix_b: u32,
    next: u32,
    stop: u32,
}

#[derive(Debug)]
pub struct Ipv6PoolString {
    address_b: u128,
    prefix_b: u128,
    next: u128,
    stop: u128,
}

impl Iterator for Ipv4PoolRaw {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.stop {
            let ret = ipv4_calculate(self.address_b, self.prefix_b, self.next);
            self.next += 1;
            Some(ret)
        } else {
            None
        }
    }
}

impl Iterator for Ipv6PoolRaw {
    type Item = Vec<u16>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.stop {
            let ret = ipv6_calculate(self.address_b, self.prefix_b, self.next);
            self.next += 1;
            Some(ret)
        } else {
            None
        }
    }
}

impl Iterator for Ipv4Pool {
    type Item = Ipv4Addr;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.stop {
            let r = ipv4_calculate(self.address_b, self.prefix_b, self.next);
            let addr = Ipv4Addr::new(r[0], r[1], r[2], r[3]);
            self.next += 1;
            Some(addr)
        } else {
            None
        }
    }
}

impl Iterator for Ipv6Pool {
    type Item = Ipv6Addr;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.stop {
            let r = ipv6_calculate(self.address_b, self.prefix_b, self.next);
            let addr = Ipv6Addr::new(r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7]);
            self.next += 1;
            Some(addr)
        } else {
            None
        }
    }
}

impl Iterator for Ipv4PoolString {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.stop {
            let r = ipv4_calculate(self.address_b, self.prefix_b, self.next);
            let addr = Ipv4Addr::new(r[0], r[1], r[2], r[3]);
            let addr_str = format!("{}", addr);
            self.next += 1;
            Some(addr_str)
        } else {
            None
        }
    }
}

impl Iterator for Ipv6PoolString {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.stop {
            let r = ipv6_calculate(self.address_b, self.prefix_b, self.next);
            let addr = Ipv6Addr::new(r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7]);
            let addr_str = format!("{}", addr);
            self.next += 1;
            Some(addr_str)
        } else {
            None
        }
    }
}

fn ipv4_calculate(address_b: u32, prefix_b: u32, next: u32) -> Vec<u8> {
    let mut ret = Vec::new();
    let subnet_b = address_b & prefix_b;
    let mut net_b = subnet_b + next;
    for _ in 0..4 {
        // ret.push((net_b & mask) as u8);
        ret.push(net_b as u8);
        net_b >>= 8;
    }
    ret.reverse();
    ret
}

fn ipv6_calculate(address_b: u128, prefix_b: u128, next: u128) -> Vec<u16> {
    let mut ret = Vec::new();
    let subnet_b = address_b & prefix_b;
    let mut net_b = subnet_b + next;
    for _ in 0..8 {
        // ret.push((net_b & mask) as u8);
        ret.push(net_b as u16);
        net_b >>= 16;
    }
    ret.reverse();
    ret
}

fn ipv4_process(tmp_address: Ipv4Addr, prefix: usize) -> (u32, u32, u32, u32) {
    let address_vec = tmp_address.octets().to_vec();
    let mut address_b: u32 = u32::MIN;
    for (i, v) in address_vec.iter().rev().enumerate() {
        // println!("{:?}:{:8b}", v, v);
        let mut v_clone = v.clone() as u32;
        for _ in 0..i {
            v_clone <<= 8;
        }
        address_b += v_clone;
    }
    let mut prefix_b: u32 = u32::MAX;
    for _ in 0..(IPV4_LEN - prefix) {
        prefix_b <<= 1;
    }
    // println!("{:32b}", address_b);
    // println!("{:32b}", prefix_b);
    // println!("{:32b}", address_b & prefix_b);
    let exp = (IPV4_LEN - prefix) as u32;
    let next = NEXT_VALUE as u32;
    let stop = u32::pow(2, exp);
    (address_b, prefix_b, next, stop)
}

fn ipv6_process(tmp_address: Ipv6Addr, prefix: usize) -> (u128, u128, u128, u128) {
    let address_vec = tmp_address.segments().to_vec();
    let mut address_b: u128 = u128::MIN;
    for (i, v) in address_vec.iter().rev().enumerate() {
        // println!("{:?}:{:8b}", v, v);
        let mut v_clone = v.clone() as u128;
        for _ in 0..i {
            v_clone <<= 16;
        }
        address_b += v_clone;
    }
    let mut prefix_b: u128 = u128::MAX;
    for _ in 0..(IPV6_LEN - prefix) {
        prefix_b <<= 1;
    }
    // println!("{:32b}", address_b);
    // println!("{:32b}", prefix_b);
    // println!("{:32b}", address_b & prefix_b);
    let exp = (IPV6_LEN - prefix) as u32;
    let next = NEXT_VALUE as u128;
    let stop = u128::pow(2, exp);
    (address_b, prefix_b, next, stop)
}

/// Returns an IPv4 subnet iterator of type Vec<u8>.
pub fn ipv4_iter_raw(subnet_address: &str) -> Option<Ipv4PoolRaw> {
    match ipv4_subnet_split(subnet_address) {
        Some((tmp_address, prefix)) => {
            let (address_b, prefix_b, next, stop) = ipv4_process(tmp_address, prefix);
            Some(Ipv4PoolRaw {
                address_b,
                prefix_b,
                next,
                stop,
            })
        }
        _ => None,
    }
}

/// Returns an IPv6 subnet iterator of type Vec<u16>.
pub fn ipv6_iter_raw(subnet_address: &str) -> Option<Ipv6PoolRaw> {
    match ipv6_subnet_split(subnet_address) {
        Some((tmp_address, prefix)) => {
            let (address_b, prefix_b, next, stop) = ipv6_process(tmp_address, prefix);
            Some(Ipv6PoolRaw {
                address_b,
                prefix_b,
                next,
                stop,
            })
        }
        _ => None,
    }
}

/// Returns an IPv4 iterator of type Ipv4Addr.
pub fn ipv4_iter(subnet_address: &str) -> Option<Ipv4Pool> {
    match ipv4_subnet_split(subnet_address) {
        Some((tmp_address, prefix)) => {
            let (address_b, prefix_b, next, stop) = ipv4_process(tmp_address, prefix);
            Some(Ipv4Pool {
                address_b,
                prefix_b,
                next,
                stop,
            })
        }
        _ => None,
    }
}

/// Returns an IPv6 iterator of type Ipv6Addr.
pub fn ipv6_iter(subnet_address: &str) -> Option<Ipv6Pool> {
    match ipv6_subnet_split(subnet_address) {
        Some((tmp_address, prefix)) => {
            let (address_b, prefix_b, next, stop) = ipv6_process(tmp_address, prefix);
            Some(Ipv6Pool {
                address_b,
                prefix_b,
                next,
                stop,
            })
        }
        _ => None,
    }
}

/// Returns an IPv4 iterator of type String.
pub fn ipv4_iter_string(subnet_address: &str) -> Option<Ipv4PoolString> {
    match ipv4_subnet_split(subnet_address) {
        Some((tmp_address, prefix)) => {
            let (address_b, prefix_b, next, stop) = ipv4_process(tmp_address, prefix);
            Some(Ipv4PoolString {
                address_b,
                prefix_b,
                next,
                stop,
            })
        }
        _ => None,
    }
}

/// Returns an IPv6 iterator of type String.
pub fn ipv6_iter_string(subnet_address: &str) -> Option<Ipv6PoolString> {
    match ipv6_subnet_split(subnet_address) {
        Some((tmp_address, prefix)) => {
            let (address_b, prefix_b, next, stop) = ipv6_process(tmp_address, prefix);
            Some(Ipv6PoolString {
                address_b,
                prefix_b,
                next,
                stop,
            })
        }
        _ => None,
    }
}

fn ipv4_to_u32(tmp_address: Ipv4Addr) -> u32 {
    let address_vec = tmp_address.octets().to_vec();
    let mut address_b: u32 = u32::MIN;
    for (i, v) in address_vec.iter().rev().enumerate() {
        // println!("{:?}:{:8b}", v, v);
        let mut v_clone = v.clone() as u32;
        for _ in 0..i {
            v_clone <<= 8;
        }
        address_b += v_clone;
    }
    address_b
}

fn ipv6_to_u128(tmp_address: Ipv6Addr) -> u128 {
    let address_vec = tmp_address.segments().to_vec();
    let mut address_b: u128 = u128::MIN;
    for (i, v) in address_vec.iter().rev().enumerate() {
        // println!("{:?}:{:8b}", v, v);
        let mut v_clone = v.clone() as u128;
        for _ in 0..i {
            v_clone <<= 16;
        }
        address_b += v_clone;
    }
    address_b
}

fn ipv4_prefix_mask(prefix: usize) -> u32 {
    let mut prefix_b: u32 = u32::MAX;
    for _ in 0..(IPV4_LEN - prefix) {
        prefix_b <<= 1;
    }
    prefix_b
}

fn ipv6_prefix_mask(prefix: usize) -> u128 {
    let mut prefix_b: u128 = u128::MAX;
    for _ in 0..(IPV6_LEN - prefix) {
        prefix_b <<= 1;
    }
    prefix_b
}

fn ipv4_subnet_split(subnet_address: &str) -> Option<(Ipv4Addr, usize)> {
    if subnet_address.contains("/") {
        let subnet_address_vec: Vec<&str> = subnet_address.split("/").collect();
        if subnet_address_vec.len() == 2 {
            let subnet = subnet_address_vec[0].parse().unwrap();
            let prefix: usize = subnet_address_vec[1].parse().unwrap();
            return Some((subnet, prefix));
        }
    }
    eprintln!("Error: Wrong subnet address");
    None
}

fn ipv6_subnet_split(subnet_address: &str) -> Option<(Ipv6Addr, usize)> {
    if subnet_address.contains("/") {
        let subnet_address_vec: Vec<&str> = subnet_address.split("/").collect();
        if subnet_address_vec.len() == 2 {
            let subnet = subnet_address_vec[0].parse().unwrap();
            let prefix: usize = subnet_address_vec[1].parse().unwrap();
            return Some((subnet, prefix));
        }
    }
    eprintln!("Error: Wrong subnet address");
    None
}

/// Check if the ip is within a subnet
pub fn ipv4_within_subnet(subnet_address: &str, address: &str) -> bool {
    // subnet_address: 192.168.1.0/24
    // address: 192.168.1.22
    let (subnet, prefix) = ipv4_subnet_split(subnet_address).unwrap();
    let subnet_b = ipv4_to_u32(subnet);
    let prefix_b = ipv4_prefix_mask(prefix);
    let address_b = ipv4_to_u32(address.parse().unwrap());
    if subnet_b & prefix_b == address_b & prefix_b {
        return true;
    }
    false
}

/// Check if the ip is within a subnet
pub fn ipv6_within_subnet(subnet_address: &str, address: &str) -> bool {
    let (subnet, prefix) = ipv6_subnet_split(subnet_address).unwrap();
    let subnet_b = ipv6_to_u128(subnet);
    let prefix_b = ipv6_prefix_mask(prefix);
    let address_b = ipv6_to_u128(address.parse().unwrap());
    if subnet_b & prefix_b == address_b & prefix_b {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reverse() {
        let mut v = vec![1, 2, 3];
        v.reverse();
        // println!("{:?}", v);
        assert!(v == vec![3, 2, 1]);
    }
    #[test]
    fn ipv4_pool_raw() {
        let ret = ipv4_iter_raw("192.168.1.0/24").unwrap();
        // println!("{:?}", ret);
        for r in ret {
            println!("{:?}", r);
            let addr = Ipv4Addr::new(r[0], r[1], r[2], r[3]);
            println!("{:?}", addr);
        }
        assert_eq!(1, 1);
    }
    #[test]
    fn ipv6_pool_raw() {
        let ret = ipv6_iter_raw("::ffff:192.10.2.255/124").unwrap();
        // println!("{:?}", ret);
        for r in ret {
            println!("{:?}", r);
            let addr = Ipv6Addr::new(r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7]);
            println!("{:?}", addr);
        }
        assert_eq!(1, 1);
    }
    #[test]
    fn ipv4_pool() {
        let ret = ipv4_iter("192.168.1.0/24").unwrap();
        // println!("{:?}", ret);
        for r in ret {
            println!("{:?}", r);
        }
        assert_eq!(1, 1);
    }
    #[test]
    fn ipv6_pool() {
        let ret = ipv6_iter("::ffff:192.10.2.255/124").unwrap();
        // println!("{:?}", ret);
        for r in ret {
            println!("{:?}", r);
        }
        assert_eq!(1, 1);
    }
    #[test]
    fn ipv4_pool_string() {
        let ret = ipv4_iter_string("192.168.1.0/24").unwrap();
        // println!("{:?}", ret);
        for r in ret {
            println!("{:?}", r);
        }
        assert_eq!(1, 1);
    }
    #[test]
    fn ipv6_pool_string() {
        let ret = ipv6_iter_string("::ffff:192.10.2.255/124").unwrap();
        // println!("{:?}", ret);
        for r in ret {
            println!("{:?}", r);
        }
        assert_eq!(1, 1);
    }
    #[test]
    fn ipv6() {
        let addr = Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff);
        println!("{:?}", addr);
        assert_eq!(1, 1);
    }
    #[test]
    fn ipv4_within_test_1() {
        let ret = ipv4_within_subnet("192.168.1.0/24", "192.168.1.200");
        println!("{:?}", ret);
        assert_eq!(true, ret);
    }
    #[test]
    fn ipv4_within_test_2() {
        let ret = ipv4_within_subnet("192.168.1.0/24", "10.8.0.22");
        println!("{:?}", ret);
        assert_eq!(false, ret);
    }
}
