use std::{net::{Ipv4Addr, Ipv6Addr, AddrParseError}, str::FromStr};

const INIT_NEXT_VALUE: usize = 1;
const IPV4_LEN: usize = 32;
const IPV6_LEN: usize = 128;

#[derive(Debug)]
pub struct Ipv4Pool {
    pub prefix: u32,
    pub next: u32,
    pub stop: u32,
}

#[derive(Debug)]
pub struct Ipv4 {
    pub addr: u32,
}

#[derive(Debug)]
pub struct Ipv6Pool {
    pub prefix: u128,
    pub next: u128,
    pub stop: u128,
}

#[derive(Debug)]
pub struct Ipv6 {
    pub addr: u128,
}

impl Iterator for Ipv4Pool {
    type Item = Ipv4Addr;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.stop {
            let ret = self.prefix + self.next;
            self.next += 1;
            Some(ret.into())
        } else {
            None
        }
    }
}

impl Iterator for Ipv6Pool {
    type Item = Ipv6Addr;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.stop {
            let ret = self.prefix + self.next;
            self.next += 1;
            Some(ret.into())
        } else {
            None
        }
    }
}

impl Ipv4 {
    /// Constructs a new `Ipv4` from a given `&str`.
    pub fn new(address: &str) -> Result<Ipv4, AddrParseError> {
        // address: 192.168.1.1
        match Ipv4Addr::from_str(address) {
            Ok(addr) => {
                let addr: u32 = addr.into();
                Ok(Ipv4 { addr })
            }
            Err(e) => Err(e),
        }
    }
    /// Returns an Ipv4 iterator over the addresses contained in the network.
    ///
    /// # Example
    /// ```
    /// use subnetwork::Ipv4;
    /// 
    /// fn main() {
    ///     let ipv4 = Ipv4::new("192.168.1.1").unwrap();
    ///     for i in ipv4.iter(24) {
    ///         println!("{:?}", i);
    ///     }
    /// }
    /// ```
    pub fn iter(&self, prefix: usize) -> Ipv4Pool {
        let mut mask: u32 = u32::MAX;
        for _ in 0..(IPV4_LEN - prefix) {
            mask <<= 1;
        }
        let exp = (IPV4_LEN - prefix) as u32;
        let next = INIT_NEXT_VALUE as u32;
        let stop = u32::pow(2, exp);
        let prefix = self.addr & mask;
        Ipv4Pool { prefix, next, stop }
    }
    /// Check if the ip is within a subnet,
    /// # Example
    /// ```
    /// use subnetwork::Ipv4;
    /// 
    /// fn main() {
    ///     let ipv4 = Ipv4::new("192.168.1.1").unwrap();
    ///     let ret = ipv4.within("192.168.1.0/24");
    ///     assert_eq!(true, ret);
    /// }
    /// ```
    pub fn within(&self, subnet_address: &str) -> bool {
        // subnet_address: 192.168.1.0/24
        let (subnet, subnet_mask) = self.subnet_split(subnet_address).unwrap();
        let subnet_u32: u32 = subnet.into();
        let prefix_u32: u32 = self.mask(subnet_mask);
        let address_u32: u32 = self.addr;
        if subnet_u32 & prefix_u32 == address_u32 & prefix_u32 {
            return true;
        } else {
            false
        }
    }
    fn mask(&self, prefix: usize) -> u32 {
        let mut prefix_b: u32 = u32::MAX;
        for _ in 0..(IPV4_LEN - prefix) {
            prefix_b <<= 1;
        }
        prefix_b
    }
    fn subnet_split(&self, subnet_address: &str) -> Option<(Ipv4Addr, usize)> {
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
}

impl Ipv6 {
    /// Constructs a new `Ipv6` from a given `&str`.
    pub fn new(address: &str) -> Result<Ipv6, AddrParseError> {
        match Ipv6Addr::from_str(address) {
            Ok(addr) => {
                let addr: u128 = addr.into();
                Ok(Ipv6 { addr })
            }
            Err(e) => Err(e),
        }
    }
    /// Returns an Ipv6 iterator over the addresses contained in the network.
    ///
    /// # Example
    /// ```
    /// use subnetwork::Ipv6;
    /// 
    /// fn main() {
    ///     let ipv6 = Ipv6::new("::ffff:192.10.2.255").unwrap();
    ///     for i in ipv6.iter(124) {
    ///         println!("{:?}", i);
    ///     }
    /// }
    /// ```
    pub fn iter(&self, prefix: usize) -> Ipv6Pool {
        let mut mask: u128 = u128::MAX;
        for _ in 0..(IPV6_LEN - prefix) {
            mask <<= 1;
        }
        let exp = (IPV6_LEN - prefix) as u32;
        let next = INIT_NEXT_VALUE as u128;
        let stop = u128::pow(2, exp);
        let prefix = self.addr & mask;
        Ipv6Pool { prefix, next, stop }
    }
    /// Check if the ip is within a subnet,
    /// # Example
    /// ```
    /// use subnetwork::Ipv6;
    /// 
    /// fn main() {
    ///     let ipv6 = Ipv6::new("::ffff:192.10.2.255").unwrap();
    ///     let ret = ipv6.within("::ffff:192.10.2.255/120");
    ///     assert_eq!(true, ret);
    /// }
    /// ```
    pub fn within(&self, subnet_address: &str) -> bool {
        let (subnet, subnet_mask) = self.subnet_split(subnet_address).unwrap();
        let subnet_u32: u128 = subnet.into();
        let prefix_u32: u128 = self.mask(subnet_mask);
        let address_u32: u128 = self.addr;
        if subnet_u32 & prefix_u32 == address_u32 & prefix_u32 {
            return true;
        } else {
            false
        }
    }
    fn mask(&self, prefix: usize) -> u128 {
        let mut prefix_b: u128 = u128::MAX;
        for _ in 0..(IPV6_LEN - prefix) {
            prefix_b <<= 1;
        }
        prefix_b
    }
    fn subnet_split(&self, subnet_address: &str) -> Option<(Ipv6Addr, usize)> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ipv4_pool() {
        let ipv4 = Ipv4::new("192.168.1.1").unwrap();
        for i in ipv4.iter(24) {
            println!("{:?}", i);
        }
        assert_eq!(1, 1);
    }
    #[test]
    fn ipv6_pool() {
        let ipv6 = Ipv6::new("::ffff:192.10.2.255").unwrap();
        for i in ipv6.iter(124) {
            println!("{:?}", i);
        }
        assert_eq!(1, 1);
    }
    #[test]
    fn ipv4() {
        let ipv4 = Ipv4::new("192.168.1.1").unwrap();
        println!("{:?}", ipv4);
        assert_eq!(ipv4.addr, 3232235777);
    }
    #[test]
    fn ipv6() {
        let ipv6 = Ipv6::new("::ffff:192.10.2.255").unwrap();
        println!("{:?}", ipv6);
        assert_eq!(ipv6.addr, 281473903624959);
    }
    #[test]
    fn ipv4_within_test_1() {
        let ipv4 = Ipv4::new("192.168.1.1").unwrap();
        let ret = ipv4.within("192.168.1.0/24");
        println!("{:?}", ret);
        assert_eq!(true, ret);
    }
    #[test]
    fn ipv4_within_test_2() {
        let ipv4 = Ipv4::new("10.8.0.22").unwrap();
        let ret = ipv4.within("192.168.1.0/24");
        println!("{:?}", ret);
        assert_eq!(false, ret);
    }
    #[test]
    fn ipv6_within_test_1() {
        let ipv6 = Ipv6::new("::ffff:192.10.2.255").unwrap();
        let ret = ipv6.within("::ffff:192.10.2.255/120");
        println!("{:?}", ret);
        assert_eq!(true, ret);
    }
}