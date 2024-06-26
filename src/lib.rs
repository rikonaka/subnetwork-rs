//! The `subnetwork` crate provides a set of APIs to work with IP CIDRs in Rust.
use std::error::Error;
use std::fmt;
use std::net::{AddrParseError, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

const INIT_NEXT_VALUE: u8 = 1;
const IPV4_LEN: u8 = 32;
const IPV6_LEN: u8 = 128;

#[derive(Debug)]
pub struct InvalidInputError {
    msg: String,
}

impl fmt::Display for InvalidInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error: invalid input [{}]", self.msg)
    }
}

impl Error for InvalidInputError {
    fn description(&self) -> &str {
        &self.msg
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CrossIpv4Pool {
    pub start: u32,
    pub end: u32,
    pub next: u32,
}

impl Iterator for CrossIpv4Pool {
    type Item = Ipv4Addr;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next <= self.end {
            let ret = self.next;
            self.next += 1;
            Some(ret.into())
        } else {
            None
        }
    }
}

impl fmt::Display for CrossIpv4Pool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let start: Ipv4Addr = self.start.into();
        let end: Ipv4Addr = self.end.into();
        write!(f, "{}-{}", start, end)
    }
}

impl CrossIpv4Pool {
    /// Returns an Ipv4 iterator over the cross different subnetwork addresses.
    ///
    /// # Example
    /// ```
    /// use subnetwork::CrossIpv4Pool;
    /// use std::net::Ipv4Addr;
    ///
    /// fn main() {
    ///     let start = Ipv4Addr::new(192, 168, 1, 1);
    ///     let end = Ipv4Addr::new(192, 168, 3, 254);
    ///     let ips = CrossIpv4Pool::new(start, end).unwrap();
    ///     for i in ips {
    ///         println!("{:?}", i);
    ///     }
    /// }
    /// ```
    pub fn new(start: Ipv4Addr, end: Ipv4Addr) -> Result<CrossIpv4Pool, InvalidInputError> {
        let start_ipv4 = Ipv4::new(start);
        let end_ipv4 = Ipv4::new(end);
        if start_ipv4.addr <= end_ipv4.addr {
            let cip = CrossIpv4Pool {
                start: start_ipv4.addr,
                end: end_ipv4.addr,
                next: start_ipv4.addr,
            };
            Ok(cip)
        } else {
            let msg = format!("{}-{}", start, end);
            Err(InvalidInputError { msg })
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ipv4Pool {
    pub prefix: u32,
    pub mask: u32,
    pub next: u32,
    pub stop: u32,
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

impl fmt::Display for Ipv4Pool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix: Ipv4Addr = self.prefix.into();
        let mut prefix_length = 0;
        let mut mask = self.mask;
        while mask != 0 {
            mask <<= 1;
            prefix_length += 1;
        }
        write!(f, "{}/{}", prefix, prefix_length)
    }
}

impl Ipv4Pool {
    /// Returns an Ipv4 iterator over the addresses contained in the network.
    ///
    /// # Example
    /// ```
    /// use subnetwork::Ipv4Pool;
    /// use std::net::Ipv4Addr;
    ///
    /// fn main() {
    ///     let ip = Ipv4Addr::new(192, 168, 1, 1);
    ///     let ips = Ipv4Pool::new(ip, 24).unwrap();
    ///     for i in ips {
    ///         println!("{:?}", i);
    ///     }
    /// }
    /// ```
    pub fn new(address: Ipv4Addr, prefix_length: u8) -> Result<Ipv4Pool, InvalidInputError> {
        if prefix_length > 32 {
            let error_addr = format!("{}/{}", address, prefix_length);
            Err(InvalidInputError { msg: error_addr })
        } else {
            let addr: u32 = address.into();
            let mut mask: u32 = u32::MAX;
            for _ in 0..(IPV4_LEN - prefix_length) {
                mask <<= 1;
            }
            let exp = (IPV4_LEN - prefix_length) as u32;
            let next = INIT_NEXT_VALUE as u32;
            let stop = u32::pow(2, exp);
            let prefix = addr & mask;
            return Ok(Ipv4Pool {
                prefix,
                mask,
                next,
                stop,
            });
        }
    }
    /// Returns an Ipv4 iterator over the addresses contained in the network.
    ///
    /// # Example
    /// ```
    /// use subnetwork::Ipv4Pool;
    ///
    /// fn main() {
    ///     let ips = Ipv4Pool::from("192.168.1.0/24").unwrap();
    ///     for i in ips {
    ///         println!("{:?}", i);
    ///     }
    /// }
    /// ```
    pub fn from(address: &str) -> Result<Ipv4Pool, InvalidInputError> {
        if address.contains("/") {
            let address_vec: Vec<&str> = address.split("/").collect();
            if address_vec.len() == 2 {
                let addr: Ipv4Addr = address_vec[0].parse().unwrap();
                let addr: u32 = addr.into();
                let prefix_length: u8 = address_vec[1].parse().unwrap();
                let mut mask: u32 = u32::MAX;
                for _ in 0..(IPV4_LEN - prefix_length) {
                    mask <<= 1;
                }
                let exp = (IPV4_LEN - prefix_length) as u32;
                let next = INIT_NEXT_VALUE as u32;
                let stop = u32::pow(2, exp);
                let prefix = addr & mask;
                return Ok(Ipv4Pool {
                    prefix,
                    mask,
                    next,
                    stop,
                });
            }
        }
        Err(InvalidInputError {
            msg: address.to_string(),
        })
    }
    /// Check if ip pool contains this ip.
    ///
    /// # Example
    /// ```
    /// use subnetwork::Ipv4Pool;
    ///
    /// fn main() {
    ///     let ips = Ipv4Pool::from("192.168.1.0/24").unwrap();
    ///     let ret = ips.contain_from_str("192.168.1.20").unwrap();
    ///     assert_eq!(ret, true);
    /// }
    /// ```
    pub fn contain_from_str(&self, address: &str) -> Result<bool, AddrParseError> {
        match Ipv4Addr::from_str(address) {
            Ok(addr) => {
                let addr: u32 = addr.into();
                if addr & self.mask == self.prefix {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(e) => Err(e),
        }
    }
    /// Check if ip pool contains this ip.
    ///
    /// # Example
    /// ```
    /// use std::net::Ipv4Addr;
    /// use std::str::FromStr;
    /// use subnetwork::Ipv4Pool;
    ///
    /// fn main() {
    ///     let ips = Ipv4Pool::from("192.168.1.0/24").unwrap();
    ///     let ip = Ipv4Addr::from_str("192.168.1.20").unwrap();
    ///     let ret = ips.contain(ip);
    ///     assert_eq!(ret, true);
    /// }
    /// ```
    pub fn contain(&self, address: Ipv4Addr) -> bool {
        let addr: u32 = address.into();
        if addr & self.mask == self.prefix {
            true
        } else {
            false
        }
    }
    /// Returns the address of the network denoted by this `Ipv4Pool`.
    /// This means the lowest possible IP address inside of the network.
    pub fn network(&self) -> Ipv4Addr {
        self.prefix.into()
    }
    /// Returns the broadcasting address of this `Ipv4Pool`.
    /// This means the highest possible IP address inside of the network.
    pub fn broadcast(&self) -> Ipv4Addr {
        let biggest = !self.mask;
        let ret = self.prefix + biggest;
        ret.into()
    }
    /// Returns the number of possible addresses in this `Ipv4Pool` (include 0 and 255)
    pub fn size(&self) -> usize {
        let biggest = !self.mask + 1;
        biggest as usize
    }
    /// Returns the number of valid addresses in this `Ipv4Pool` (NOT include 0 and 255)
    pub fn len(&self) -> usize {
        let length = !self.mask - 1;
        length as usize
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CrossIpv6Pool {
    pub start: u128,
    pub end: u128,
    pub next: u128,
}

impl Iterator for CrossIpv6Pool {
    type Item = Ipv6Addr;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next <= self.end {
            let ret = self.next;
            self.next += 1;
            Some(ret.into())
        } else {
            None
        }
    }
}

impl fmt::Display for CrossIpv6Pool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let start: Ipv6Addr = self.start.into();
        let end: Ipv6Addr = self.end.into();
        write!(f, "{}-{}", start, end)
    }
}

impl CrossIpv6Pool {
    /// Returns an Ipv4 iterator over the cross different subnetwork addresses.
    ///
    /// # Example
    /// ```
    /// use subnetwork::CrossIpv6Pool;
    /// use std::net::Ipv6Addr;
    ///
    /// fn main() {
    ///     let start_str = "fe80::215:5dff:fe20:b393";
    ///     let end_str = "fe80::215:5dff:fe20:b395";
    ///     let start: Ipv6Addr = start_str.parse().unwrap();
    ///     let end: Ipv6Addr = end_str.parse().unwrap();
    ///     let ips = CrossIpv6Pool::new(start, end).unwrap();
    ///     for i in ips {
    ///         println!("{:?}", i);
    ///     }
    /// }
    /// ```
    pub fn new(start: Ipv6Addr, end: Ipv6Addr) -> Result<CrossIpv6Pool, InvalidInputError> {
        let start_ipv6 = Ipv6::new(start);
        let end_ipv6 = Ipv6::new(end);
        if start_ipv6.addr <= end_ipv6.addr {
            let cip = CrossIpv6Pool {
                start: start_ipv6.addr,
                end: end_ipv6.addr,
                next: start_ipv6.addr,
            };
            Ok(cip)
        } else {
            let msg = format!("{}-{}", start, end);
            Err(InvalidInputError { msg })
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ipv6Pool {
    pub prefix: u128,
    pub mask: u128,
    pub next: u128,
    pub stop: u128,
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

impl fmt::Display for Ipv6Pool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix: Ipv6Addr = self.prefix.into();
        let mut prefix_length = 0;
        let mut mask = self.mask;
        while mask != 0 {
            mask <<= 1;
            prefix_length += 1;
        }
        write!(f, "{}/{}", prefix, prefix_length)
    }
}

impl Ipv6Pool {
    /// Returns an Ipv6 iterator over the addresses contained in the network.
    ///
    /// # Example
    /// ```
    /// use subnetwork::Ipv6Pool;
    /// use std::net::Ipv6Addr;
    ///
    /// fn main() {
    ///     let ipv6_str = "::ffff:192.10.2.0";
    ///     let ipv6: Ipv6Addr = ipv6_str.parse().unwrap();
    ///     let ips = Ipv6Pool::new(ipv6, 120).unwrap();
    ///     for i in ips {
    ///         println!("{:?}", i);
    ///     }
    /// }
    /// ```
    pub fn new(address: Ipv6Addr, prefix_length: u8) -> Result<Ipv6Pool, InvalidInputError> {
        if prefix_length > 128 {
            let error_addr = format!("{}/{}", address, prefix_length);
            Err(InvalidInputError { msg: error_addr })
        } else {
            let addr: u128 = address.into();
            let mut mask: u128 = u128::MAX;
            for _ in 0..(IPV6_LEN - prefix_length) {
                mask <<= 1;
            }
            let exp = (IPV6_LEN - prefix_length) as u32;
            let next = INIT_NEXT_VALUE as u128;
            let stop = u128::pow(2, exp);
            let prefix = addr & mask;
            return Ok(Ipv6Pool {
                prefix,
                mask,
                next,
                stop,
            });
        }
    }
    /// Returns an Ipv6 iterator over the addresses contained in the network.
    ///
    /// # Example
    /// ```
    /// use subnetwork::Ipv6Pool;
    ///
    /// fn main() {
    ///     let ips = Ipv6Pool::from("::ffff:192.10.2.0/120").unwrap();
    ///     for i in ips {
    ///         println!("{:?}", i);
    ///     }
    /// }
    /// ```
    pub fn from(address: &str) -> Result<Ipv6Pool, InvalidInputError> {
        if address.contains("/") {
            let address_vec: Vec<&str> = address.split("/").collect();
            if address_vec.len() == 2 {
                let addr: Ipv6Addr = address_vec[0].parse().unwrap();
                let addr: u128 = addr.into();
                let prefix_length: u8 = address_vec[1].parse().unwrap();
                let mut mask: u128 = u128::MAX;
                for _ in 0..(IPV6_LEN - prefix_length) {
                    mask <<= 1;
                }
                let exp = (IPV6_LEN - prefix_length) as u32;
                let next = INIT_NEXT_VALUE as u128;
                let stop = u128::pow(2, exp);
                let prefix = addr & mask;
                return Ok(Ipv6Pool {
                    prefix,
                    mask,
                    next,
                    stop,
                });
            }
        }
        Err(InvalidInputError {
            msg: address.to_string(),
        })
    }
    /// Check if ip pool contains this ip.
    ///
    /// # Example
    /// ```
    /// use subnetwork::Ipv6Pool;
    ///
    /// fn main() {
    ///     let ips = Ipv6Pool::from("::ffff:192.10.2.0/120").unwrap();
    ///     let ret = ips.contain_from_str("::ffff:192.10.2.1").unwrap();
    ///     assert_eq!(ret, true);
    /// }
    /// ```
    pub fn contain_from_str(&self, address: &str) -> Result<bool, AddrParseError> {
        match Ipv6Addr::from_str(address) {
            Ok(addr) => {
                let addr: u128 = addr.into();
                if addr & self.mask == self.prefix {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(e) => Err(e),
        }
    }
    /// Check if ip pool contains this ip.
    ///
    /// # Example
    /// ```
    /// use std::net::Ipv6Addr;
    /// use std::str::FromStr;
    /// use subnetwork::Ipv6Pool;
    ///
    /// fn main() {
    ///     let ips = Ipv6Pool::from("::ffff:192.10.2.0/120").unwrap();
    ///     let ip = Ipv6Addr::from_str("::ffff:192.10.2.1").unwrap();
    ///     let ret = ips.contain(ip);
    ///     assert_eq!(ret, true);
    /// }
    /// ```
    pub fn contain(&self, address: Ipv6Addr) -> bool {
        let addr: u128 = address.into();
        if addr & self.mask == self.prefix {
            true
        } else {
            false
        }
    }
    /// Returns the address of the network denoted by this `Ipv6Pool`.
    /// This means the lowest possible IP address inside of the network.
    pub fn network(&self) -> Ipv6Addr {
        self.prefix.into()
    }
    /// Returns the number of possible host addresses in this `Ipv6Pool` (include 0 and 255)
    pub fn size(&self) -> usize {
        let biggest = !self.mask + 1;
        biggest as usize
    }
    /// Returns the number of valid addresses in this `Ipv6Pool` (NOT include 0 and 255)
    pub fn len(&self) -> usize {
        let length = !self.mask - 1;
        length as usize
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ipv4 {
    pub addr: u32,
}

impl fmt::Display for Ipv4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let addr: Ipv4Addr = self.addr.into();
        write!(f, "{}", addr)
    }
}

impl Ipv4 {
    /// Constructs a new `Ipv4` from a given Ipv4Addr.
    pub fn new(address: Ipv4Addr) -> Ipv4 {
        // address: 192.168.1.1
        let addr: u32 = address.into();
        Ipv4 { addr }
    }
    /// Constructs a new `Ipv4` from a given `&str`.
    pub fn from(address: &str) -> Result<Ipv4, AddrParseError> {
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
    ///     let ipv4 = Ipv4::from("192.168.1.1").unwrap();
    ///     for i in ipv4.iter(24) {
    ///         println!("{:?}", i);
    ///     }
    /// }
    /// ```
    pub fn iter(&self, prefix_length: u8) -> Ipv4Pool {
        let mut mask: u32 = u32::MAX;
        for _ in 0..(IPV4_LEN - prefix_length) {
            mask <<= 1;
        }
        let exp = (IPV4_LEN - prefix_length) as u32;
        let next = INIT_NEXT_VALUE as u32;
        let stop = u32::pow(2, exp);
        let prefix = self.addr & mask;
        Ipv4Pool {
            prefix,
            mask,
            next,
            stop,
        }
    }
    /// Check if the ip is within a subnet.
    /// # Example
    /// ```
    /// use subnetwork::Ipv4;
    ///
    /// fn main() {
    ///     let ipv4 = Ipv4::from("192.168.1.1").unwrap();
    ///     let ret = ipv4.within_from_str("192.168.1.0/24").unwrap();
    ///     assert_eq!(ret, true);
    /// }
    /// ```
    pub fn within_from_str(&self, subnet_address: &str) -> Result<bool, InvalidInputError> {
        match self.subnet_split(subnet_address) {
            Ok((subnet, subnet_mask)) => {
                let new_subnet_address: u32 = subnet.into();
                let new_subnet_mask: u32 = self.get_subnet_mask(subnet_mask);
                if new_subnet_address & new_subnet_mask == self.addr & new_subnet_mask {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(e) => Err(e),
        }
    }
    /// Check if the ip is within a subnet.
    /// # Example
    /// ```
    /// use subnetwork::{Ipv4, Ipv4Pool};
    ///
    /// fn main() {
    ///     let ipv4 = Ipv4::from("192.168.1.1").unwrap();
    ///     let ipv4_pool = Ipv4Pool::from("192.168.1.0/24").unwrap();
    ///     let ret = ipv4.within(ipv4_pool);
    ///     assert_eq!(ret, true);
    /// }
    /// ```
    pub fn within(&self, subnet_address: Ipv4Pool) -> bool {
        let addr = self.addr;
        if addr & subnet_address.mask == subnet_address.prefix {
            true
        } else {
            false
        }
    }
    /// Returns the address of the network denoted by this `Ipv4`.
    /// This means the lowest possible IP address inside of the network.
    pub fn network(&self, prefix_length: u8) -> Ipv4Addr {
        let mut mask: u32 = u32::MAX;
        for _ in 0..(IPV4_LEN - prefix_length) {
            mask <<= 1;
        }
        let ret = self.addr & mask;
        ret.into()
    }
    /// Returns the broadcasting address of this `Ipv4`.
    /// This means the highest possible IP address inside of the network.
    pub fn broadcast(&self, prefix_length: u8) -> Ipv4Addr {
        let mut mask: u32 = u32::MAX;
        for _ in 0..(IPV4_LEN - prefix_length) {
            mask <<= 1;
        }
        let prefix = self.addr & mask;
        let exp = (IPV4_LEN - prefix_length) as u32;
        let biggest = u32::pow(2, exp) - 1;
        let ret = prefix + biggest;
        ret.into()
    }
    /// Returns the number of possible host addresses in this `Ipv4` (include 0 and 255)
    pub fn size(&self, prefix_length: u8) -> usize {
        let exp = (IPV4_LEN - prefix_length) as u32;
        let biggest = u32::pow(2, exp);
        biggest as usize
    }
    /// Returns the number of valid addresses in this `Ipv4` (NOT include 0 and 255)
    pub fn len(&self, prefix_length: u8) -> usize {
        let exp = (IPV4_LEN - prefix_length) as u32;
        let length = u32::pow(2, exp) - 2;
        length as usize
    }
    /// Returns the standard IPv4 address.
    pub fn to_std(&self) -> Ipv4Addr {
        self.addr.into()
    }
    fn get_subnet_mask(&self, prefix_length: u8) -> u32 {
        let mut mask: u32 = u32::MAX;
        for _ in 0..(IPV4_LEN - prefix_length) {
            mask <<= 1;
        }
        mask
    }
    fn subnet_split(&self, subnet_address: &str) -> Result<(Ipv4Addr, u8), InvalidInputError> {
        if subnet_address.contains("/") {
            let subnet_address_vec: Vec<&str> = subnet_address.split("/").collect();
            if subnet_address_vec.len() == 2 {
                let subnet = subnet_address_vec[0].parse().unwrap();
                let prefix: u8 = subnet_address_vec[1].parse().unwrap();
                return Ok((subnet, prefix));
            }
        }
        Err(InvalidInputError {
            msg: subnet_address.to_string(),
        })
    }
    /// Returns the largest identical prefix of two IP addresses.
    /// # Example
    /// ```
    /// use subnetwork::{Ipv4, Ipv4Pool};
    ///
    /// fn main() {
    ///     let ipv4_1 = Ipv4::from("192.168.1.136").unwrap();
    ///     let ipv4_2 = Ipv4::from("192.168.1.192").unwrap();
    ///     let ret = ipv4_1.largest_identical_prefix(ipv4_2);
    ///     assert_eq!(ret, 25);
    /// }
    /// ```
    pub fn largest_identical_prefix(&self, target: Ipv4) -> u32 {
        let a = self.addr;
        let b = target.addr;
        let mut mask = 1;
        for _ in 0..31 {
            mask <<= 1;
        }
        let mut count = 0;
        for _ in 0..32 {
            if a & mask != b & mask {
                break;
            }
            count += 1;
            mask >>= 1;
        }
        count
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ipv6 {
    pub addr: u128,
}

impl fmt::Display for Ipv6 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let addr: Ipv6Addr = self.addr.into();
        write!(f, "{}", addr)
    }
}

impl Ipv6 {
    /// Constructs a new `Ipv6` from a given Ipv6Addr.
    pub fn new(address: Ipv6Addr) -> Ipv6 {
        let addr: u128 = address.into();
        Ipv6 { addr }
    }
    /// Constructs a new `Ipv6` from a given `&str`.
    pub fn from(address: &str) -> Result<Ipv6, AddrParseError> {
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
    ///     let ipv6 = Ipv6::from("::ffff:192.10.2.255").unwrap();
    ///     for i in ipv6.iter(124) {
    ///         println!("{:?}", i);
    ///     }
    /// }
    /// ```
    pub fn iter(&self, prefix_length: u8) -> Ipv6Pool {
        let mut mask: u128 = u128::MAX;
        for _ in 0..(IPV6_LEN - prefix_length) {
            mask <<= 1;
        }
        let exp = (IPV6_LEN - prefix_length) as u32;
        let next = INIT_NEXT_VALUE as u128;
        let stop = u128::pow(2, exp);
        let prefix = self.addr & mask;
        Ipv6Pool {
            prefix,
            mask,
            next,
            stop,
        }
    }
    /// Check if the ip is within a subnet.
    /// # Example
    /// ```
    /// use subnetwork::Ipv6;
    ///
    /// fn main() {
    ///     let ipv6 = Ipv6::from("::ffff:192.10.2.255").unwrap();
    ///     let ret = ipv6.within_from_str("::ffff:192.10.2.255/120").unwrap();
    ///     assert_eq!(ret, true);
    /// }
    /// ```
    pub fn within_from_str(&self, subnet_address: &str) -> Result<bool, InvalidInputError> {
        match self.subnet_split(subnet_address) {
            Ok((subnet, subnet_mask)) => {
                let new_subnet_address: u128 = subnet.into();
                let new_subnet_mask: u128 = self.get_subnet_mask(subnet_mask);
                if new_subnet_address & new_subnet_mask == self.addr & new_subnet_mask {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(e) => Err(e),
        }
    }
    /// Check if the ip is within a subnet.
    /// # Example
    /// ```
    /// use subnetwork::{Ipv6, Ipv6Pool};
    ///
    /// fn main() {
    ///     let ipv6 = Ipv6::from("::ffff:192.10.2.255").unwrap();
    ///     let ipv6_pool = Ipv6Pool::from("::ffff:192.10.2.255/120").unwrap();
    ///     let ret = ipv6.within(ipv6_pool);
    ///     assert_eq!(ret, true);
    /// }
    /// ```
    pub fn within(&self, subnet_address: Ipv6Pool) -> bool {
        let addr = self.addr;
        if addr & subnet_address.mask == subnet_address.prefix {
            true
        } else {
            false
        }
    }
    /// Returns the address of the network denoted by this `Ipv6`.
    /// This means the lowest possible IP address inside of the network.
    pub fn network(&self, prefix_length: u8) -> Ipv6Addr {
        let mut mask: u128 = u128::MAX;
        for _ in 0..(IPV6_LEN - prefix_length) {
            mask <<= 1;
        }
        let ret = self.addr & mask;
        ret.into()
    }
    /// Returns the node local scope multicast address of this `Ipv6`.
    pub fn node_multicast(&self) -> Ipv6Addr {
        let node = Ipv6Addr::new(
            0xFF01, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001, 0xFF00, 0x0000,
        );
        let node = Ipv6::new(node);
        let mask = Ipv6Addr::new(
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x00FF, 0xFFFF,
        );
        let mask = Ipv6::new(mask);
        (node.addr + (mask.addr & self.addr)).into()
    }
    /// Returns the link local scope multicast address of this `Ipv6`.
    pub fn link_multicast(&self) -> Ipv6Addr {
        let link = Ipv6Addr::new(
            0xFF02, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001, 0xFF00, 0x0000,
        );
        let link = Ipv6::new(link);
        let mask = Ipv6Addr::new(
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x00FF, 0xFFFF,
        );
        let mask = Ipv6::new(mask);
        (link.addr + (mask.addr & self.addr)).into()
    }
    /// Returns the site local scope multicast address of this `Ipv6`.
    pub fn site_multicast(&self) -> Ipv6Addr {
        let site = Ipv6Addr::new(
            0xFF05, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001, 0xFF00, 0x0000,
        );
        let site = Ipv6::new(site);
        let mask = Ipv6Addr::new(
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x00FF, 0xFFFF,
        );
        let mask = Ipv6::new(mask);
        (site.addr + (mask.addr & self.addr)).into()
    }
    /// Returns the number of possible host addresses in this `Ipv6`
    pub fn size(&self, prefix_length: u8) -> usize {
        let exp = (IPV6_LEN - prefix_length) as u32;
        let biggest = u128::pow(2, exp);
        biggest as usize
    }
    /// Returns the number of valid addresses in this `Ipv6` (NOT include 0 and 255)
    pub fn len(&self, prefix_length: u8) -> usize {
        let exp = (IPV6_LEN - prefix_length) as u32;
        let length = u128::pow(2, exp) - 2;
        length as usize
    }
    /// Returns the standard IPv4 address.
    pub fn to_std(&self) -> Ipv6Addr {
        self.addr.into()
    }
    fn get_subnet_mask(&self, prefix_length: u8) -> u128 {
        let mut mask: u128 = u128::MAX;
        for _ in 0..(IPV6_LEN - prefix_length) {
            mask <<= 1;
        }
        mask
    }
    fn subnet_split(&self, subnet_address: &str) -> Result<(Ipv6Addr, u8), InvalidInputError> {
        if subnet_address.contains("/") {
            let subnet_address_vec: Vec<&str> = subnet_address.split("/").collect();
            if subnet_address_vec.len() == 2 {
                let subnet = subnet_address_vec[0].parse().unwrap();
                let prefix: u8 = subnet_address_vec[1].parse().unwrap();
                return Ok((subnet, prefix));
            }
        }
        Err(InvalidInputError {
            msg: subnet_address.to_string(),
        })
    }
    pub fn max_identical_prefix(&self, target: Ipv6) -> u128 {
        let a = self.addr;
        let b = target.addr;
        let mut mask = 1;
        for _ in 0..127 {
            mask <<= 1;
        }
        let mut count = 0;
        for _ in 0..128 {
            if a & mask != b & mask {
                break;
            }
            count += 1;
            mask >>= 1;
        }
        count - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /******************** cross ipv4 ********************/
    #[test]
    fn cross_ipv4_pool_print() {
        let start = Ipv4Addr::new(192, 168, 1, 1);
        let end = Ipv4Addr::new(192, 168, 3, 254);
        let ips = CrossIpv4Pool::new(start, end).unwrap();
        for i in ips {
            println!("{:?}", i);
        }
    }
    /******************** ipv4 ********************/
    #[test]
    fn ipv4_pool_print() {
        let test_str = "192.168.1.0/24";
        let ipv4_pool = Ipv4Pool::from(test_str).unwrap();
        let ipv4_pool_str = format!("{}", ipv4_pool);
        assert_eq!(ipv4_pool_str, test_str);
    }
    #[test]
    fn ipv4_print() {
        let test_str = "192.168.1.1";
        let ipv4 = Ipv4::from(test_str).unwrap();
        let ipv4_str = format!("{}", ipv4);
        assert_eq!(ipv4_str, test_str);
    }
    #[test]
    fn ipv4_iter() {
        let ipv4 = Ipv4::from("192.168.1.1").unwrap();
        for i in ipv4.iter(24) {
            println!("{:?}", i);
        }
        assert_eq!(1, 1);
    }
    #[test]
    fn ipv6_iter() {
        let ipv6 = Ipv6::from("::ffff:192.10.2.255").unwrap();
        for i in ipv6.iter(124) {
            println!("{:?}", i);
        }
        assert_eq!(1, 1);
    }
    #[test]
    fn ipv4() {
        let ipv4 = Ipv4::from("192.168.1.1").unwrap();
        println!("{:8b}", ipv4.addr);
        assert_eq!(ipv4.addr, 3232235777);
    }
    #[test]
    fn ipv4_within_test_1() {
        let ipv4 = Ipv4::from("192.168.1.1").unwrap();
        let ret = ipv4.within_from_str("192.168.1.0/24").unwrap();
        println!("{:?}", ret);
        assert_eq!(ret, true);
    }
    #[test]
    fn ipv4_within_test_2() {
        let ipv4 = Ipv4::from("10.8.0.22").unwrap();
        let ret = ipv4.within_from_str("192.168.1.0/24").unwrap();
        println!("{:?}", ret);
        assert_eq!(ret, false);
    }
    #[test]
    fn ipv4_network() {
        let ipv4 = Ipv4::from("192.168.1.1").unwrap();
        let ipv4_2 = Ipv4Addr::new(192, 168, 1, 0);
        println!("{:?}", ipv4.network(24));
        assert_eq!(ipv4.network(24), ipv4_2);
    }
    #[test]
    fn ipv4_broadcast() {
        let ipv4 = Ipv4::from("192.168.1.1").unwrap();
        let ipv4_2 = Ipv4Addr::new(192, 168, 1, 255);
        println!("{:?}", ipv4.broadcast(24));
        assert_eq!(ipv4.broadcast(24), ipv4_2);
    }
    #[test]
    fn ipv4_size() {
        let ipv4 = Ipv4::from("192.168.1.1").unwrap();
        let subnet_size = ipv4.size(24);
        println!("{:?}", subnet_size);
        assert_eq!(subnet_size, 256);
    }
    #[test]
    fn ipv4_len() {
        let ipv4 = Ipv4::from("192.168.1.1").unwrap();
        let subnet_size = ipv4.len(24);
        println!("{:?}", subnet_size);
        assert_eq!(subnet_size, 254);
    }
    /******************** ipv6 ********************/
    #[test]
    fn ipv6() {
        let ipv6 = Ipv6::from("::ffff:192.10.2.255").unwrap();
        println!("{:?}", ipv6);
        assert_eq!(ipv6.addr, 281473903624959);
    }
    #[test]
    fn ipv6_within_test_1() {
        let ipv6 = Ipv6::from("::ffff:192.10.2.255").unwrap();
        let ret = ipv6.within_from_str("::ffff:192.10.2.255/120").unwrap();
        println!("{:?}", ret);
        assert_eq!(ret, true);
    }
    #[test]
    fn ipv6_network() {
        let ipv6 = Ipv6::from("::ffff:192.10.2.255").unwrap();
        let ipv6_2: Ipv6Addr = "::ffff:192.10.2.0".parse().unwrap();
        println!("{:?}", ipv6.network(120));
        assert_eq!(ipv6.network(120), ipv6_2);
    }
    #[test]
    fn ipv6_node() {
        // let a: u8 = 0b1100;
        // let b: u8 = 0b0011;
        // println!("{}", a + b);
        let ipv6 = Ipv6::from("::ffff:192.10.2.255").unwrap();
        let ipv6_2: Ipv6Addr = "ff01::1:ff0a:2ff".parse().unwrap();
        println!("{:?}", ipv6.node_multicast());
        assert_eq!(ipv6.node_multicast(), ipv6_2);
    }
    #[test]
    fn ipv6_link() {
        let ipv6 = Ipv6::from("::ffff:192.10.2.255").unwrap();
        let ipv6_2: Ipv6Addr = "ff02::1:ff0a:2ff".parse().unwrap();
        println!("{:?}", ipv6.link_multicast());
        assert_eq!(ipv6.link_multicast(), ipv6_2);
    }
    #[test]
    fn ipv6_size() {
        let ipv6 = Ipv6::from("::ffff:192.10.2.255").unwrap();
        let subnet_size = ipv6.size(120);
        println!("{:?}", subnet_size);
        assert_eq!(subnet_size, 256);
    }
    #[test]
    fn ipv6_len() {
        let ipv6 = Ipv6::from("::ffff:192.10.2.255").unwrap();
        let subnet_len = ipv6.len(120);
        println!("{:?}", subnet_len);
        assert_eq!(subnet_len, 254);
    }
    /******************** ipv4 pool ********************/
    #[test]
    fn ipv4_pool() {
        let ips = Ipv4Pool::from("192.168.1.0/24").unwrap();
        for i in ips {
            println!("{:?}", i);
        }
        assert_eq!(1, 1);
    }
    #[test]
    fn ipv4_pool_new() {
        let ip = Ipv4Addr::new(192, 168, 1, 1);
        let ips = Ipv4Pool::new(ip, 24).unwrap();
        for i in ips {
            println!("{:?}", i);
        }
        assert_eq!(1, 1);
    }
    #[test]
    fn ipv4_pool_contain_1() {
        let ips = Ipv4Pool::from("192.168.1.0/24").unwrap();
        let ret = ips.contain_from_str("192.168.1.20").unwrap();
        println!("{:?}", ret);
        assert_eq!(ret, true);
    }
    #[test]
    fn ipv4_pool_contain_2() {
        let ips = Ipv4Pool::from("192.168.1.0/24").unwrap();
        let ret = ips.contain_from_str("10.8.0.20").unwrap();
        println!("{:?}", ret);
        assert_eq!(ret, false);
    }
    #[test]
    fn ipv4_pool_network() {
        let ips = Ipv4Pool::from("192.168.1.0/24").unwrap();
        let network = ips.network();
        let network_2 = Ipv4Addr::new(192, 168, 1, 0);
        println!("{:?}", network);
        assert_eq!(network, network_2);
    }
    #[test]
    fn ipv4_pool_broadcast() {
        let ips = Ipv4Pool::from("192.168.1.0/24").unwrap();
        let broadcast = ips.broadcast();
        let broadcast_2 = Ipv4Addr::new(192, 168, 1, 255);
        println!("{:?}", broadcast);
        assert_eq!(broadcast, broadcast_2);
    }
    #[test]
    fn ipv4_pool_size() {
        let ips = Ipv4Pool::from("192.168.1.0/24").unwrap();
        let size = ips.size();
        println!("{:?}", size);
        assert_eq!(size, 256);
    }
    #[test]
    fn ipv4_pool_len() {
        let ips = Ipv4Pool::from("192.168.1.0/24").unwrap();
        let size = ips.len();
        println!("{:?}", size);
        assert_eq!(size, 254);
    }
    #[test]
    fn test_largest_identical_prefix() {
        let ipv4_1 = Ipv4::from("192.168.1.136").unwrap();
        let ipv4_2 = Ipv4::from("192.168.1.192").unwrap();
        let ret = ipv4_1.largest_identical_prefix(ipv4_2);
        println!("{}", ret);
    }
    #[test]
    fn test_max_idt() {
        let a: u32 = 14;
        let b: u32 = 12;
        let mut mask = 1;
        for _ in 0..31 {
            mask <<= 1;
        }
        println!("{}", mask);

        let mut count = 0;
        for _ in 0..32 {
            if a & mask != b & mask {
                break;
            }
            count += 1;
            mask >>= 1;
        }
        println!("{}", count);
    }
}
