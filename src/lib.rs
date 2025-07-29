//! The `subnetwork` crate provides a set of APIs to work with IP CIDRs in Rust.
use std::fmt;
use std::net::AddrParseError;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::num::ParseIntError;
use std::result;
use std::str::FromStr;
use thiserror::Error;

const INIT_NEXT_VALUE: u8 = 0;
const IPV4_PREFIX_MAX_LEN: u8 = 32;
const IPV6_PREFIX_MAX_LEN: u8 = 128;

pub type Result<T, E = SubnetworkError> = result::Result<T, E>;

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
    /// Returns an Ipv4 iterator over the cross different subnetwork.
    /// # Example
    /// ```
    /// use subnetwork::CrossIpv4Pool;
    /// use std::net::Ipv4Addr;
    ///
    /// fn main() {
    ///     let start = Ipv4Addr::new(192, 168, 1, 1);
    ///     let end = Ipv4Addr::new(192, 168, 3, 254);
    ///     let pool = CrossIpv4Pool::new(start, end).unwrap();
    ///     for i in pool {
    ///         println!("{:?}", i);
    ///     }
    /// }
    /// ```
    pub fn new<T: Into<Ipv4AddrExt>>(
        start: T,
        end: Ipv4Addr,
    ) -> Result<CrossIpv4Pool, SubnetworkError> {
        let start_ip_ext: Ipv4AddrExt = start.into();
        let end_ip_ext: Ipv4AddrExt = end.into();
        let start_u32: u32 = start_ip_ext.addr;
        let end_u32: u32 = end_ip_ext.addr;

        if start_u32 <= end_u32 {
            let cip = CrossIpv4Pool {
                start: start_u32,
                end: end_u32,
                next: start_u32,
            };
            Ok(cip)
        } else {
            let error_range = format!("{}-{}", start_u32, end_u32);
            Err(SubnetworkError::InvalidInput { msg: error_range })
        }
    }
    /// Extract all IPs.
    pub fn to_vec(&self) -> Vec<Ipv4Addr> {
        self.into_iter().collect()
    }
    /// Check if ip pool contains this ip.
    pub fn contain(&self, addr: Ipv4Addr) -> bool {
        let addr: u32 = addr.into();
        if addr <= self.end && addr >= self.start {
            true
        } else {
            false
        }
    }
    /// Returns the number of possible host address in this `CrossIpv4Pool`.
    pub fn len(&self) -> usize {
        let length = self.end - self.start;
        length as usize
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
        let prefix_addr: Ipv4Addr = self.prefix.into();
        let mut prefix = 0;
        let mut mask = self.mask;
        while mask != 0 {
            mask <<= 1;
            prefix += 1;
        }
        let now_addr = self.prefix + self.next;
        let now_addr: Ipv4Addr = now_addr.into();
        write!(f, "{}/{}, next {}", prefix_addr, prefix, now_addr)
    }
}

impl FromStr for Ipv4Pool {
    type Err = SubnetworkError;
    fn from_str(addr: &str) -> Result<Self, Self::Err> {
        if addr.contains("/") {
            let addr_vec: Vec<&str> = addr.split("/").collect();
            if addr_vec.len() == 2 {
                let ip_addr = Ipv4Addr::from_str(addr_vec[0])?;
                let prefix = u8::from_str(addr_vec[1])?;
                if prefix <= IPV4_PREFIX_MAX_LEN {
                    let addr: u32 = ip_addr.into();
                    let mask: u32 = u32::MAX << (IPV4_PREFIX_MAX_LEN - prefix);
                    let next = INIT_NEXT_VALUE as u32;
                    let stop = 1 << (IPV4_PREFIX_MAX_LEN - prefix);
                    let prefix = addr & mask;
                    return Ok(Ipv4Pool {
                        prefix,
                        mask,
                        next,
                        stop,
                    });
                }
            }
        }
        // final
        Err(SubnetworkError::InvalidInput {
            msg: addr.to_string(),
        })
    }
}

impl Ipv4Pool {
    /// Returns an Ipv4 iterator over the address contained in the network.
    /// Include network address and broadcast address.
    /// # Example
    /// ```
    /// use subnetwork::Ipv4Pool;
    /// use std::net::Ipv4Addr;
    ///
    /// fn main() {
    ///     let ip = Ipv4Addr::new(192, 168, 1, 1);
    ///     let pool = Ipv4Pool::new(ip, 24).unwrap();
    ///     for i in pool {
    ///         println!("{:?}", i);
    ///     }
    /// }
    /// ```
    pub fn new<T: Into<Ipv4AddrExt>>(addr: T, prefix: u8) -> Result<Ipv4Pool, SubnetworkError> {
        let addr_ext: Ipv4AddrExt = addr.into();
        if prefix > IPV4_PREFIX_MAX_LEN {
            let error_addr = format!("{}/{}", addr_ext, prefix);
            Err(SubnetworkError::InvalidInput {
                msg: error_addr.to_string(),
            })
        } else {
            let addr: u32 = addr_ext.addr;
            let mask: u32 = u32::MAX << (IPV4_PREFIX_MAX_LEN - prefix);
            let next = INIT_NEXT_VALUE as u32;
            let stop = 1 << (IPV4_PREFIX_MAX_LEN - prefix);
            let prefix = addr & mask;
            return Ok(Ipv4Pool {
                prefix,
                mask,
                next,
                stop,
            });
        }
    }
    /// Extract all IPs.
    pub fn to_vec(&self) -> Vec<Ipv4Addr> {
        self.into_iter().collect()
    }
    /// Check if ip pool contains this ip.
    /// # Example
    /// ```
    /// use std::net::Ipv4Addr;
    /// use std::str::FromStr;
    /// use subnetwork::Ipv4Pool;
    ///
    /// fn main() {
    ///     let pool = Ipv4Pool::from_str("192.168.1.0/24").unwrap();
    ///     let ip = Ipv4Addr::from_str("192.168.1.20").unwrap();
    ///     let ret = pool.contain(ip);
    ///     assert_eq!(ret, true);
    /// }
    /// ```
    pub fn contain(&self, addr: Ipv4Addr) -> bool {
        let addr: u32 = addr.into();
        if addr & self.mask == self.prefix {
            true
        } else {
            false
        }
    }
    /// Returns the addr of the network denoted by this `Ipv4Pool`.
    /// This means the lowest possible IP addr inside of the network.
    pub fn network(&self) -> Ipv4Addr {
        self.prefix.into()
    }
    /// Returns the broadcasting addr of this `Ipv4Pool`.
    /// This means the highest possible IP addr inside of the network.
    pub fn broadcast(&self) -> Ipv4Addr {
        let biggest = !self.mask;
        let ret = self.prefix + biggest;
        ret.into()
    }
    /// Returns the number of possible address in this `Ipv4Pool` (include 0 and 255).
    pub fn len(&self) -> usize {
        let biggest = !self.mask + 1;
        biggest as usize
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
    /// Returns an Ipv4 iterator over the cross different subnetwork address.
    /// # Example
    /// ```
    /// use subnetwork::CrossIpv6Pool;
    /// use std::net::Ipv6Addr;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     let start = Ipv6Addr::from_str("fe80::215:5dff:fe20:b393").unwrap();
    ///     let end = Ipv6Addr::from_str("fe80::215:5dff:fe20:b395").unwrap();
    ///     let pool = CrossIpv6Pool::new(start, end).unwrap();
    ///     for i in pool {
    ///         println!("{:?}", i);
    ///     }
    /// }
    /// ```
    pub fn new(start: Ipv6Addr, end: Ipv6Addr) -> Result<CrossIpv6Pool, SubnetworkError> {
        let start_ipv6: Ipv6AddrExt = start.into();
        let end_ipv6: Ipv6AddrExt = end.into();
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
    /// Extract all IPs.
    pub fn to_vec(&self) -> Vec<Ipv6Addr> {
        self.into_iter().collect()
    }
    /// Check if ip pool contains this ip.
    pub fn contain(&self, addr: Ipv6Addr) -> bool {
        let addr: u128 = addr.into();
        if addr <= self.end && addr >= self.start {
            true
        } else {
            false
        }
    }
    /// Returns the number of possible host address in this `CrossIpv6Pool`.
    pub fn len(&self) -> usize {
        let length = self.end - self.start;
        length as usize
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
        let prefix_addr: Ipv6Addr = self.prefix.into();
        let mut prefix = 0;
        let mut mask = self.mask;
        while mask != 0 {
            mask <<= 1;
            prefix += 1;
        }
        write!(f, "{}/{}", prefix_addr, prefix)
    }
}

impl FromStr for Ipv6Pool {
    type Err = SubnetworkError;
    fn from_str(addr: &str) -> Result<Self, Self::Err> {
        if addr.contains("/") {
            let addr_vec: Vec<&str> = addr.split("/").collect();
            if addr_vec.len() == 2 {
                let ip_addr = Ipv6Addr::from_str(addr_vec[0])?;
                let prefix = u8::from_str(addr_vec[1])?;
                if prefix <= IPV6_PREFIX_MAX_LEN {
                    let addr: u128 = ip_addr.into();
                    let mask: u128 = u128::MAX << (IPV6_PREFIX_MAX_LEN - prefix);
                    let next = INIT_NEXT_VALUE as u128;
                    let stop = 1 << (IPV6_PREFIX_MAX_LEN - prefix);
                    let prefix = addr & mask;
                    return Ok(Ipv6Pool {
                        prefix,
                        mask,
                        next,
                        stop,
                    });
                }
            }
        }
        // final
        Err(SubnetworkError::InvalidInput {
            msg: addr.to_string(),
        })
    }
}

impl Ipv6Pool {
    /// Returns an Ipv6 iterator over the address contained in the network.
    /// Include network address and broadcast address.
    /// # Example
    /// ```
    /// use subnetwork::Ipv6Pool;
    /// use std::net::Ipv6Addr;
    ///
    /// fn main() {
    ///     let ipv6_str = "::ffff:192.10.2.0";
    ///     let ipv6: Ipv6Addr = ipv6_str.parse().unwrap();
    ///     let pool = Ipv6Pool::new(ipv6, 120).unwrap();
    ///     for i in pool {
    ///         println!("{:?}", i);
    ///     }
    /// }
    /// ```
    pub fn new(addr: Ipv6Addr, prefix: u8) -> Result<Ipv6Pool, SubnetworkError> {
        if prefix > IPV6_PREFIX_MAX_LEN {
            let error_addr = format!("{}/{}", addr, prefix);
            Err(SubnetworkError::InvalidInput {
                msg: error_addr.to_string(),
            })
        } else {
            let addr: u128 = addr.into();
            let mask: u128 = u128::MAX << (IPV6_PREFIX_MAX_LEN - prefix);
            let next = INIT_NEXT_VALUE as u128;
            let stop = 1 << (IPV6_PREFIX_MAX_LEN - prefix);
            let prefix = addr & mask;
            Ok(Ipv6Pool {
                prefix,
                mask,
                next,
                stop,
            })
        }
    }
    /// Extract all IPs.
    pub fn to_vec(&self) -> Vec<Ipv6Addr> {
        self.into_iter().collect()
    }
    /// Check if ip pool contains this ip.
    /// # Example
    /// ```
    /// use std::net::Ipv6Addr;
    /// use std::str::FromStr;
    /// use subnetwork::Ipv6Pool;
    ///
    /// fn main() {
    ///     let pool = Ipv6Pool::from_str("::ffff:192.10.2.0/120").unwrap();
    ///     let ip = Ipv6Addr::from_str("::ffff:192.10.2.1").unwrap();
    ///     let ret = pool.contain(ip);
    ///     assert_eq!(ret, true);
    /// }
    /// ```
    pub fn contain(&self, addr: Ipv6Addr) -> bool {
        let addr: u128 = addr.into();
        if addr & self.mask == self.prefix {
            true
        } else {
            false
        }
    }
    /// Returns the addr of the network denoted by this `Ipv6Pool`.
    /// This means the lowest possible IP addr inside of the network.
    pub fn network(&self) -> Ipv6Addr {
        self.prefix.into()
    }
    /// Returns the number of possible host address in this `Ipv6Pool`.
    pub fn len(&self) -> usize {
        let biggest = !self.mask + 1;
        biggest as usize
    }
}

/* Single Addr Struct */

#[derive(Debug, Clone, Copy)]
pub struct Ipv4AddrExt {
    addr: u32,
}

impl fmt::Display for Ipv4AddrExt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let addr: Ipv4Addr = self.addr.into();
        write!(f, "{}", addr)
    }
}

impl From<Ipv4Addr> for Ipv4AddrExt {
    fn from(addr: Ipv4Addr) -> Self {
        let addr: u32 = addr.into();
        Ipv4AddrExt { addr }
    }
}

impl From<Ipv4AddrExt> for Ipv4Addr {
    fn from(addr: Ipv4AddrExt) -> Self {
        let new_addr: u32 = addr.addr;
        new_addr.into()
    }
}

impl FromStr for Ipv4AddrExt {
    type Err = SubnetworkError;
    fn from_str(addr: &str) -> Result<Self, Self::Err> {
        let new_addr = Ipv4Addr::from_str(addr)?;
        let addr: u32 = new_addr.into();
        Ok(Ipv4AddrExt { addr })
    }
}

impl Ipv4AddrExt {
    /// Creates a new IPv4AddrExt from four eight-bit octets.
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Ipv4AddrExt {
        let a_fix = (a as u32) << 24;
        let b_fix = (b as u32) << 16;
        let c_fix = (c as u32) << 8;
        let d_fix = d as u32;
        let addr = a_fix + b_fix + c_fix + d_fix;
        Ipv4AddrExt { addr }
    }
    /// Returns the largest identical prefix of two IP address.
    /// # Example
    /// ```
    /// use subnetwork::Ipv4AddrExt;
    /// use subnetwork::Ipv4Pool;
    /// use std::net::Ipv4Addr;
    ///
    /// fn main() {
    ///     let ipv4_1 = Ipv4Addr::new(192, 168, 1, 136);
    ///     let ipv4_2 = Ipv4Addr::new(192, 168, 1, 192);
    ///     let ipv4ext_1: Ipv4AddrExt = ipv4_1.into();
    ///     let ret = ipv4ext_1.largest_identical_prefix(ipv4_2);
    ///     assert_eq!(ret, 25);
    /// }
    /// ```
    pub fn largest_identical_prefix<T: Into<Ipv4AddrExt>>(&self, target: T) -> u8 {
        let a = self.addr;
        let b = target.into().addr;
        let init_mask = 2u32.pow(31);
        let mut mask = init_mask;

        for c in 0..IPV4_PREFIX_MAX_LEN {
            if a & mask != b & mask {
                return c;
            }
            mask = (mask >> 1) + init_mask;
        }
        0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ipv6AddrExt {
    addr: u128,
}

impl fmt::Display for Ipv6AddrExt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let addr: Ipv6Addr = self.addr.into();
        write!(f, "{}", addr)
    }
}

impl From<Ipv6Addr> for Ipv6AddrExt {
    fn from(addr: Ipv6Addr) -> Self {
        let addr: u128 = addr.into();
        Ipv6AddrExt { addr }
    }
}

impl From<Ipv6AddrExt> for Ipv6Addr {
    fn from(addr: Ipv6AddrExt) -> Self {
        let new_addr: u128 = addr.addr;
        new_addr.into()
    }
}

impl FromStr for Ipv6AddrExt {
    type Err = SubnetworkError;
    fn from_str(addr: &str) -> Result<Self, Self::Err> {
        let new_addr = Ipv6Addr::from_str(addr)?;
        let addr: u128 = new_addr.into();
        Ok(Ipv6AddrExt { addr })
    }
}

impl Ipv6AddrExt {
    /// Creates a new IPv6 address from eight 16-bit segments.
    pub fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Ipv6AddrExt {
        let a_fix = (a as u128) << 112;
        let b_fix = (b as u128) << 96;
        let c_fix = (c as u128) << 80;
        let d_fix = (d as u128) << 64;
        let e_fix = (e as u128) << 48;
        let f_fix = (f as u128) << 32;
        let g_fix = (g as u128) << 16;
        let h_fix = h as u128;
        let addr = a_fix + b_fix + c_fix + d_fix + e_fix + f_fix + g_fix + h_fix;
        Ipv6AddrExt { addr }
    }
    /// Returns the node local scope multicast addr of this `Ipv6`.
    pub fn node_multicast(&self) -> Ipv6Addr {
        let node = Ipv6Addr::new(
            0xff01, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001, 0xff00, 0x0000,
        );
        let mask = Ipv6Addr::new(
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x00ff, 0xffff,
        );
        let node_u128: u128 = node.into();
        let mask_u128: u128 = mask.into();
        (node_u128 + (mask_u128 & self.addr)).into()
    }
    /// Returns the link local scope multicast addr of this `Ipv6`.
    pub fn link_multicast(&self) -> Ipv6Addr {
        let link = Ipv6Addr::new(
            0xff02, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001, 0xff00, 0x0000,
        );
        let mask = Ipv6Addr::new(
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x00ff, 0xffff,
        );
        let link_u128: u128 = link.into();
        let mask_u128: u128 = mask.into();
        (link_u128 + (mask_u128 & self.addr)).into()
    }
    /// Returns the site local scope multicast addr of this `Ipv6`.
    pub fn site_multicast(&self) -> Ipv6Addr {
        let site = Ipv6Addr::new(
            0xff05, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001, 0xff00, 0x0000,
        );
        let mask = Ipv6Addr::new(
            0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x00ff, 0xffff,
        );
        let site_u128: u128 = site.into();
        let mask_u128: u128 = mask.into();
        (site_u128 + (mask_u128 & self.addr)).into()
    }
    pub fn largest_identical_prefix<T: Into<Ipv6AddrExt>>(&self, target: T) -> u8 {
        let a = self.addr;
        let b = target.into().addr;
        let init_mask = 2u128.pow(127);
        let mut mask = init_mask;

        for c in 0..IPV6_PREFIX_MAX_LEN {
            if a & mask != b & mask {
                return c;
            }
            mask = (mask >> 1) + init_mask;
        }
        0
    }
}

pub struct NetmaskExt {
    prefix: u8,
}

impl NetmaskExt {
    /// Constructs a new `Ipv6` from a given `&str`.
    /// # Example
    /// ```
    /// use subnetwork::NetmaskExt;
    ///
    /// fn main() {
    ///     let netmask = NetmaskExt::new(24);
    ///     // 255.255.255.0
    ///     let netmask_ip = netmask.to_ipv4().unwrap();
    /// }
    /// ```
    pub fn new(prefix: u8) -> NetmaskExt {
        NetmaskExt { prefix }
    }
    pub fn to_ipv4(&self) -> Result<Ipv4Addr, SubnetworkError> {
        if self.prefix == 0 {
            Ok((0 as u32).into())
        } else {
            if self.prefix > IPV4_PREFIX_MAX_LEN {
                let msg = format!("prefix: {}", self.prefix);
                Err(SubnetworkError::InvalidInput { msg })
            } else {
                Ok((u32::MAX << (IPV4_PREFIX_MAX_LEN - self.prefix)).into())
            }
        }
    }
    pub fn to_ipv6(&self) -> Result<Ipv6Addr, SubnetworkError> {
        if self.prefix == 0 {
            Ok((0 as u128).into())
        } else {
            if self.prefix > IPV6_PREFIX_MAX_LEN {
                let msg = format!("prefix: {}", self.prefix);
                Err(SubnetworkError::InvalidInput { msg })
            } else {
                Ok((u128::MAX << (IPV6_PREFIX_MAX_LEN - self.prefix)).into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /* README.md examples */
    #[test]
    fn readme_example_1() {
        let pool = Ipv4Pool::new(Ipv4Addr::new(192, 168, 1, 1), 24).unwrap();
        // from 192.168.1.0 to 192.168.1.255
        for ipv4 in pool {
            println!("{}", ipv4);
        }

        let pool = Ipv4Pool::from_str("192.168.1.0/24").unwrap();
        for ipv4 in pool {
            println!("{}", ipv4);
        }

        let pool: Ipv4Pool = "192.168.1.0/24".parse().unwrap();
        for ipv4 in pool {
            println!("{}", ipv4);
        }

        let test_ipv4 = Ipv4Addr::new(192, 168, 1, 233);
        assert_eq!(pool.contain(test_ipv4), true);

        let broadcast = Ipv4Addr::new(192, 168, 1, 255);
        assert_eq!(pool.broadcast(), broadcast);

        let network = Ipv4Addr::new(192, 168, 1, 0);
        assert_eq!(pool.network(), network);

        assert_eq!(pool.len(), 256);
        // pool is copied.
        assert_eq!(pool.to_string(), "192.168.1.0/24, next 192.168.1.0");
    }
    #[test]
    fn readme_example_2() {
        let start = Ipv4Addr::new(192, 168, 1, 16);
        let end = Ipv4Addr::new(192, 168, 3, 200);
        let pool = CrossIpv4Pool::new(start, end).unwrap();
        // include 192.168.1.16 and 192.168.3.200
        for i in pool {
            println!("{:?}", i);
        }

        let test_ipv4 = Ipv4Addr::new(192, 168, 1, 233);
        assert_eq!(pool.contain(test_ipv4), true);
        let test_ipv4 = Ipv4Addr::new(192, 168, 2, 0);
        assert_eq!(pool.contain(test_ipv4), true);
        let test_ipv4 = Ipv4Addr::new(192, 168, 3, 255);
        assert_eq!(pool.contain(test_ipv4), false);
        let test_ipv4 = Ipv4Addr::new(192, 168, 3, 200);
        assert_eq!(pool.contain(test_ipv4), true);
    }
    #[test]
    fn readme_example_3() {
        // test1
        let ip1 = Ipv4Addr::new(192, 168, 1, 0);
        let ip2 = Ipv4Addr::new(192, 168, 1, 255);

        let ip1ext: Ipv4AddrExt = ip1.into();
        assert_eq!(ip1ext.largest_identical_prefix(ip2), 24);

        // test 2
        let ip1 = Ipv4Addr::new(192, 168, 1, 136);
        let ip2 = Ipv4Addr::new(192, 168, 1, 192);

        let ip1ext: Ipv4AddrExt = ip1.into();
        assert_eq!(ip1ext.largest_identical_prefix(ip2), 25);
    }
    #[test]
    fn readme_example_4() {
        let ipv6 = Ipv6Addr::from_str("::ffff:192.10.2.255").unwrap();
        let ipv6_ext: Ipv6AddrExt = ipv6.into();

        let ipv6_node_multicast = Ipv6Addr::from_str("ff01::1:ff0a:2ff").unwrap();
        assert_eq!(ipv6_ext.node_multicast(), ipv6_node_multicast);

        let ipv6_link_multicast = Ipv6Addr::from_str("ff02::1:ff0a:2ff").unwrap();
        assert_eq!(ipv6_ext.link_multicast(), ipv6_link_multicast);

        let ipv6_site_multicast = Ipv6Addr::from_str("ff05::1:ff0a:2ff").unwrap();
        assert_eq!(ipv6_ext.site_multicast(), ipv6_site_multicast);
    }
    #[test]
    fn readme_example_5() {
        let netmask = NetmaskExt::new(24);
        let netmask_addr = netmask.to_ipv4().unwrap();
        assert_eq!(netmask_addr, Ipv4Addr::new(255, 255, 255, 0));

        let netmask = NetmaskExt::new(26);
        let netmask_addr = netmask.to_ipv4().unwrap();
        assert_eq!(netmask_addr, Ipv4Addr::new(255, 255, 255, 192));
    }
    /* Others */
    #[test]
    fn ipv4_methods() {
        let ipv4 = Ipv4Addr::new(192, 168, 1, 1);
        if ipv4.is_private() {
            println!("{} is private", ipv4);
        } else {
            println!("{} is not private", ipv4);
        }
        let ipv6 = Ipv6Addr::new(0xfe80, 0, 0, 0, 0x20c, 0x29ff, 0xfedd, 0xf57);
        if ipv6.is_multicast() {
            println!("{} is multicast", ipv6);
        } else {
            println!("{} is not multicast", ipv6);
        }
    }
    /* Ipv4 */
    #[test]
    fn ipv4_pool_print() {
        let test_str = "192.168.1.0/24";
        let ipv4_pool = Ipv4Pool::from_str(test_str).unwrap();
        let ipv4_pool_str = format!("{}", ipv4_pool);
        println!("{}", ipv4_pool_str);
    }
    #[test]
    fn ipv4_print() {
        let test_str = "192.168.1.1";
        let ipv4 = Ipv4AddrExt::from_str(test_str).unwrap();
        let ipv4_str = format!("{}", ipv4);
        assert_eq!(ipv4_str, test_str);
    }
    #[test]
    fn ipv4() {
        let ipv4 = Ipv4AddrExt::from_str("192.168.1.1").unwrap();
        println!("{:8b}", ipv4.addr);
        assert_eq!(ipv4.addr, 3232235777);
    }
    /* Ipv6 */
    #[test]
    fn ipv6() {
        let ipv6 = Ipv6AddrExt::from_str("::ffff:192.10.2.255").unwrap();
        println!("{:?}", ipv6);
        assert_eq!(ipv6.addr, 281473903624959);
    }
    #[test]
    fn test_github_issues_1() {
        // return error instead of panic
        let _pool1 = Ipv4Pool::from_str("1.2.3.4/33");
        let _pool2 = Ipv4Pool::from_str("1.2.3.4/");
        let _pool3 = Ipv4Pool::from_str("nonip/24");
    }
}
