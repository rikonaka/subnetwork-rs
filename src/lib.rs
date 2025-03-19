//! The `subnetwork` crate provides a set of APIs to work with IP CIDRs in Rust.
use std::fmt;
use std::net::AddrParseError;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

const INIT_NEXT_VALUE: u8 = 1;
const IPV4_LEN: u8 = 32;
const IPV6_LEN: u8 = 128;

#[derive(Error, Debug)]
pub enum SubnetworkError {
    #[error("invalid input: {msg}")]
    InvalidInput { msg: String },
    #[error("ip addr parse error")]
    AddrParseError(#[from] AddrParseError),
    #[error("num parse error")]
    ParseIntError(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy)]
pub struct CrossIpv4Pool {
    start: u32,
    end: u32,
    next: u32,
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
        let now: Ipv4Addr = self.next.into();
        write!(f, "{}-{}, next {}", start, end, now)
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
    pub fn new(start: Ipv4Addr, end: Ipv4Addr) -> Result<CrossIpv4Pool, SubnetworkError> {
        let start_ipv4 = SubnetworkIpv4::new(start);
        let end_ipv4 = SubnetworkIpv4::new(end);
        if start_ipv4.addr <= end_ipv4.addr {
            let cip = CrossIpv4Pool {
                start: start_ipv4.addr,
                end: end_ipv4.addr,
                next: start_ipv4.addr,
            };
            Ok(cip)
        } else {
            let msg = format!("{}-{}", start, end);
            Err(SubnetworkError::InvalidInput { msg })
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ipv4Pool {
    prefix: u32,
    mask: u32,
    next: u32,
    stop: u32,
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
        let mut prefix_len = 0;
        let mut mask = self.mask;
        while mask != 0 {
            mask <<= 1;
            prefix_len += 1;
        }
        let now_addr = self.prefix + self.next;
        let now_addr: Ipv4Addr = now_addr.into();
        write!(f, "{}/{}, next {}", prefix, prefix_len, now_addr)
    }
}

impl Ipv4Pool {
    fn addr_check(ip_addr: &Ipv4Addr, prefix_len: u8) -> Result<(), SubnetworkError> {
        if prefix_len > IPV4_LEN {
            let error_addr = format!("{}/{}", ip_addr, prefix_len);
            Err(SubnetworkError::InvalidInput {
                msg: error_addr.to_string(),
            })
        } else {
            Ok(())
        }
    }
    fn addr_check_str(address: &str) -> Result<(Ipv4Addr, u8), SubnetworkError> {
        if address.contains("/") {
            let address_vec: Vec<&str> = address.split("/").collect();
            if address_vec.len() == 2 {
                let ip_addr: Ipv4Addr = address_vec[0].parse()?;
                let prefix_len: u8 = address_vec[1].parse()?;
                match Ipv4Pool::addr_check(&ip_addr, prefix_len) {
                    Ok(_) => return Ok((ip_addr, prefix_len)),
                    Err(e) => return Err(e),
                }
            }
        }
        Err(SubnetworkError::InvalidInput {
            msg: address.to_string(),
        })
    }
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
    pub fn new(address: Ipv4Addr, prefix_len: u8) -> Result<Ipv4Pool, SubnetworkError> {
        match Ipv4Pool::addr_check(&address, prefix_len) {
            Ok(_) => {
                let addr: u32 = address.into();
                let mut mask: u32 = u32::MAX;
                for _ in 0..(IPV4_LEN - prefix_len) {
                    mask <<= 1;
                }
                let exp = (IPV4_LEN - prefix_len) as u32;
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
            Err(e) => Err(e),
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
    pub fn from(address: &str) -> Result<Ipv4Pool, SubnetworkError> {
        match Ipv4Pool::addr_check_str(address) {
            Ok((ip_addr, prefix_len)) => {
                let ip_addr: u32 = ip_addr.into();
                let mut mask: u32 = u32::MAX;
                for _ in 0..(IPV4_LEN - prefix_len) {
                    mask <<= 1;
                }
                let exp = (IPV4_LEN - prefix_len) as u32;
                let next = INIT_NEXT_VALUE as u32;
                let stop = u32::pow(2, exp);
                let prefix = ip_addr & mask;
                return Ok(Ipv4Pool {
                    prefix,
                    mask,
                    next,
                    stop,
                });
            }
            Err(e) => Err(e),
        }
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
    pub fn contain_from_str(&self, address: &str) -> Result<bool, SubnetworkError> {
        match Ipv4Addr::from_str(address) {
            Ok(addr) => {
                let addr: u32 = addr.into();
                if addr & self.mask == self.prefix {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(e) => Err(e.into()),
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
    start: u128,
    end: u128,
    next: u128,
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
    pub fn new(start: Ipv6Addr, end: Ipv6Addr) -> Result<CrossIpv6Pool, SubnetworkError> {
        let start_ipv6 = SubnetworkIpv6::new(start);
        let end_ipv6 = SubnetworkIpv6::new(end);
        if start_ipv6.addr <= end_ipv6.addr {
            let cip = CrossIpv6Pool {
                start: start_ipv6.addr,
                end: end_ipv6.addr,
                next: start_ipv6.addr,
            };
            Ok(cip)
        } else {
            let msg = format!("{}-{}", start, end);
            Err(SubnetworkError::InvalidInput { msg })
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ipv6Pool {
    prefix: u128,
    mask: u128,
    next: u128,
    stop: u128,
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
        let mut prefix_len = 0;
        let mut mask = self.mask;
        while mask != 0 {
            mask <<= 1;
            prefix_len += 1;
        }
        write!(f, "{}/{}", prefix, prefix_len)
    }
}

impl Ipv6Pool {
    fn addr_check(ip_addr: &Ipv6Addr, prefix_len: u8) -> Result<(), SubnetworkError> {
        if prefix_len > IPV6_LEN {
            let error_addr = format!("{}/{}", ip_addr, prefix_len);
            Err(SubnetworkError::InvalidInput {
                msg: error_addr.to_string(),
            })
        } else {
            Ok(())
        }
    }
    fn addr_check_str(address: &str) -> Result<(Ipv6Addr, u8), SubnetworkError> {
        if address.contains("/") {
            let address_vec: Vec<&str> = address.split("/").collect();
            if address_vec.len() == 2 {
                let ip_addr: Ipv6Addr = address_vec[0].parse()?;
                let prefix_len: u8 = address_vec[1].parse()?;
                match Ipv6Pool::addr_check(&ip_addr, prefix_len) {
                    Ok(_) => return Ok((ip_addr, prefix_len)),
                    Err(e) => return Err(e),
                }
            }
        }
        Err(SubnetworkError::InvalidInput {
            msg: address.to_string(),
        })
    }
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
    pub fn new(address: Ipv6Addr, prefix_len: u8) -> Result<Ipv6Pool, SubnetworkError> {
        match Ipv6Pool::addr_check(&address, prefix_len) {
            Ok(_) => {
                let addr: u128 = address.into();
                let mut mask: u128 = u128::MAX;
                for _ in 0..(IPV6_LEN - prefix_len) {
                    mask <<= 1;
                }
                let exp = (IPV6_LEN - prefix_len) as u32;
                let next = INIT_NEXT_VALUE as u128;
                let stop = u128::pow(2, exp);
                let prefix = addr & mask;
                Ok(Ipv6Pool {
                    prefix,
                    mask,
                    next,
                    stop,
                })
            }
            Err(e) => Err(e),
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
    pub fn from(address: &str) -> Result<Ipv6Pool, SubnetworkError> {
        match Ipv6Pool::addr_check_str(address) {
            Ok((addr, prefix_len)) => {
                let addr: u128 = addr.into();
                let mut mask: u128 = u128::MAX;
                for _ in 0..(IPV6_LEN - prefix_len) {
                    mask <<= 1;
                }
                let exp = (IPV6_LEN - prefix_len) as u32;
                let next = INIT_NEXT_VALUE as u128;
                let stop = u128::pow(2, exp);
                let prefix = addr & mask;
                Ok(Ipv6Pool {
                    prefix,
                    mask,
                    next,
                    stop,
                })
            }
            Err(e) => Err(e),
        }
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
    pub fn contain_from_str(&self, address: &str) -> Result<bool, SubnetworkError> {
        match Ipv6Addr::from_str(address) {
            Ok(addr) => {
                let addr: u128 = addr.into();
                if addr & self.mask == self.prefix {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(e) => Err(e.into()),
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

/* Single Addr Struct */

#[derive(Debug, Clone, Copy)]
pub struct SubnetworkIpv4 {
    addr: u32,
}

impl fmt::Display for SubnetworkIpv4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let addr: Ipv4Addr = self.addr.into();
        write!(f, "{}", addr)
    }
}

impl SubnetworkIpv4 {
    fn prefix_len_check(&self, prefix_len: u8) -> Result<(), SubnetworkError> {
        if prefix_len > IPV4_LEN {
            let addr: Ipv4Addr = self.addr.into();
            let error_msg = format!("{}/{}", addr, prefix_len);
            Err(SubnetworkError::InvalidInput { msg: error_msg })
        } else {
            Ok(())
        }
    }
    /// Constructs a new `Ipv4` from a given Ipv4Addr.
    pub fn new(address: Ipv4Addr) -> SubnetworkIpv4 {
        // address: 192.168.1.1
        let addr: u32 = address.into();
        SubnetworkIpv4 { addr }
    }
    /// Constructs a new `Ipv4` from a given `&str`.
    ///
    /// # Example
    /// ```
    /// use subnetwork::Ipv4;
    ///
    /// fn main() {
    ///     let ipv4 = Ipv4::from("192.168.1.1").unwrap();
    ///     for i in ipv4.iter(24).unwrap() {
    ///         println!("{:?}", i);
    ///     }
    /// }
    /// ```
    pub fn from(address: &str) -> Result<SubnetworkIpv4, SubnetworkError> {
        // address: 192.168.1.1
        match Ipv4Addr::from_str(address) {
            Ok(addr) => {
                let addr: u32 = addr.into();
                Ok(SubnetworkIpv4 { addr })
            }
            Err(e) => Err(e.into()),
        }
    }
    pub fn iter(&self, prefix_len: u8) -> Result<Ipv4Pool, SubnetworkError> {
        match self.prefix_len_check(prefix_len) {
            Ok(_) => {
                let mut mask: u32 = u32::MAX;
                for _ in 0..(IPV4_LEN - prefix_len) {
                    mask <<= 1;
                }
                let exp = (IPV4_LEN - prefix_len) as u32;
                let next = INIT_NEXT_VALUE as u32;
                let stop = u32::pow(2, exp);
                let prefix = self.addr & mask;
                Ok(Ipv4Pool {
                    prefix,
                    mask,
                    next,
                    stop,
                })
            }
            Err(e) => Err(e),
        }
    }
    /// Returns the standard IPv4 address.
    pub fn to_std(&self) -> Ipv4Addr {
        self.addr.into()
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
    pub fn largest_identical_prefix(&self, target: SubnetworkIpv4) -> u32 {
        let a = self.addr;
        let b = target.addr;
        let mut mask = 1;
        for _ in 0..(IPV4_LEN - 1) {
            mask <<= 1;
        }
        let mut count = 0;
        for _ in 0..IPV4_LEN {
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
pub struct SubnetworkIpv6 {
    addr: u128,
}

impl fmt::Display for SubnetworkIpv6 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let addr: Ipv6Addr = self.addr.into();
        write!(f, "{}", addr)
    }
}

impl SubnetworkIpv6 {
    fn prefix_len_check(&self, prefix_len: u8) -> Result<(), SubnetworkError> {
        if prefix_len > IPV6_LEN {
            let addr: Ipv6Addr = self.addr.into();
            let msg = format!("{}/{}", addr, prefix_len);
            Err(SubnetworkError::InvalidInput { msg })
        } else {
            Ok(())
        }
    }
    /// Constructs a new `Ipv6` from a given Ipv6Addr.
    pub fn new(address: Ipv6Addr) -> SubnetworkIpv6 {
        let addr: u128 = address.into();
        SubnetworkIpv6 { addr }
    }
    /// Constructs a new `Ipv6` from a given `&str`.
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
    pub fn from(address: &str) -> Result<SubnetworkIpv6, SubnetworkError> {
        match Ipv6Addr::from_str(address) {
            Ok(addr) => {
                let addr: u128 = addr.into();
                Ok(SubnetworkIpv6 { addr })
            }
            Err(e) => Err(e.into()),
        }
    }
    /// Returns an Ipv6 iterator over the addresses contained in the network.
    pub fn iter(&self, prefix_len: u8) -> Result<Ipv6Pool, SubnetworkError> {
        match self.prefix_len_check(prefix_len) {
            Ok(_) => {
                let mut mask: u128 = u128::MAX;
                for _ in 0..(IPV6_LEN - prefix_len) {
                    mask <<= 1;
                }
                let exp = (IPV6_LEN - prefix_len) as u32;
                let next = INIT_NEXT_VALUE as u128;
                let stop = u128::pow(2, exp);
                let prefix = self.addr & mask;
                Ok(Ipv6Pool {
                    prefix,
                    mask,
                    next,
                    stop,
                })
            }
            Err(e) => Err(e),
        }
    }
    /// Returns the node local scope multicast address of this `Ipv6`.
    pub fn node_multicast(&self) -> Ipv6Addr {
        let node = Ipv6Addr::new(
            0xFF01, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001, 0xFF00, 0x0000,
        );
        let node = SubnetworkIpv6::new(node);
        let mask = Ipv6Addr::new(
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x00FF, 0xFFFF,
        );
        let mask = SubnetworkIpv6::new(mask);
        (node.addr + (mask.addr & self.addr)).into()
    }
    /// Returns the link local scope multicast address of this `Ipv6`.
    pub fn link_multicast(&self) -> Ipv6Addr {
        let link = Ipv6Addr::new(
            0xFF02, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001, 0xFF00, 0x0000,
        );
        let link = SubnetworkIpv6::new(link);
        let mask = Ipv6Addr::new(
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x00FF, 0xFFFF,
        );
        let mask = SubnetworkIpv6::new(mask);
        (link.addr + (mask.addr & self.addr)).into()
    }
    /// Returns the site local scope multicast address of this `Ipv6`.
    pub fn site_multicast(&self) -> Ipv6Addr {
        let site = Ipv6Addr::new(
            0xFF05, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001, 0xFF00, 0x0000,
        );
        let site = SubnetworkIpv6::new(site);
        let mask = Ipv6Addr::new(
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x00FF, 0xFFFF,
        );
        let mask = SubnetworkIpv6::new(mask);
        (site.addr + (mask.addr & self.addr)).into()
    }
    /// Returns the standard IPv4 address.
    pub fn to_std(&self) -> Ipv6Addr {
        self.addr.into()
    }
    pub fn max_identical_prefix(&self, target: SubnetworkIpv6) -> u128 {
        let a = self.addr;
        let b = target.addr;
        let mut mask = 1;
        for _ in 0..(IPV6_LEN - 1) {
            mask <<= 1;
        }
        let mut count = 0;
        for _ in 0..IPV6_LEN {
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
    /* cross ipv4 pool */
    #[test]
    fn cross_ipv4_pool_print() {
        let start = Ipv4Addr::new(192, 168, 1, 1);
        let end = Ipv4Addr::new(192, 168, 3, 254);
        let ips = CrossIpv4Pool::new(start, end).unwrap();
        for i in ips {
            println!("{:?}", i);
        }
    }
    /* ipv4 test */
    #[test]
    fn ipv4_pool_print() {
        let test_str = "192.168.1.0/24";
        let ipv4_pool = Ipv4Pool::from(test_str).unwrap();
        let ipv4_pool_str = format!("{}", ipv4_pool);
        println!("{}", ipv4_pool_str);
    }
    #[test]
    fn ipv4_print() {
        let test_str = "192.168.1.1";
        let ipv4 = SubnetworkIpv4::from(test_str).unwrap();
        let ipv4_str = format!("{}", ipv4);
        assert_eq!(ipv4_str, test_str);
    }
    #[test]
    fn ipv4_iter() {
        let ipv4 = SubnetworkIpv4::from("192.168.1.1").unwrap();
        for i in ipv4.iter(24).unwrap() {
            println!("{:?}", i);
        }
        assert_eq!(1, 1);
    }
    #[test]
    fn ipv6_iter() {
        let ipv6 = SubnetworkIpv6::from("::ffff:192.10.2.255").unwrap();
        for i in ipv6.iter(124).unwrap() {
            println!("{:?}", i);
        }
        assert_eq!(1, 1);
    }
    #[test]
    fn ipv4() {
        let ipv4 = SubnetworkIpv4::from("192.168.1.1").unwrap();
        println!("{:8b}", ipv4.addr);
        assert_eq!(ipv4.addr, 3232235777);
    }
    /* ipv6 test */
    #[test]
    fn ipv6() {
        let ipv6 = SubnetworkIpv6::from("::ffff:192.10.2.255").unwrap();
        println!("{:?}", ipv6);
        assert_eq!(ipv6.addr, 281473903624959);
    }
    #[test]
    fn ipv6_node() {
        // let a: u8 = 0b1100;
        // let b: u8 = 0b0011;
        // println!("{}", a + b);
        let ipv6 = SubnetworkIpv6::from("::ffff:192.10.2.255").unwrap();
        let ipv6_2: Ipv6Addr = "ff01::1:ff0a:2ff".parse().unwrap();
        println!("{:?}", ipv6.node_multicast());
        assert_eq!(ipv6.node_multicast(), ipv6_2);
    }
    #[test]
    fn ipv6_link() {
        let ipv6 = SubnetworkIpv6::from("::ffff:192.10.2.255").unwrap();
        let ipv6_2: Ipv6Addr = "ff02::1:ff0a:2ff".parse().unwrap();
        println!("{:?}", ipv6.link_multicast());
        assert_eq!(ipv6.link_multicast(), ipv6_2);
    }
    /* ipv4 pool test */
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
        let ipv4_1 = SubnetworkIpv4::from("192.168.1.136").unwrap();
        let ipv4_2 = SubnetworkIpv4::from("192.168.1.192").unwrap();
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
    #[test]
    // #[should_panic]
    fn test_github_issues_1() {
        let _pool1 = Ipv4Pool::from("1.2.3.4/33");
        let _pool2 = Ipv4Pool::from("1.2.3.4/");
        let _pool3 = Ipv4Pool::from("nonip/24");
    }
}
