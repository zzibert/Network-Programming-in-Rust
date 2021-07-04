#![feature(ip)]

use std::net::{IpAddr, SocketAddr};

fn main() {
    // construct an IpAddr from a string and check it
    // represents the loopback address
    let local: IpAddr = "127.0.0.1".parse().unwrap();
    assert!(local.is_loopback());

    // constructs a globally routable IPv6 address from individual octects
    // and assert it is classified correctly

    let global: IpAddr = IpAddr::from([0, 0, 0x1c9, 0, 0, 0xafc8, 0, 0x1]);
    assert!(global.is_global());

    // constructs a SocketAddr from a string an assert that the underlying
    // IP is a IPv4 address
    let local_sa: SocketAddr = "127.0.0.1:80".parse().unwrap();
    assert!(local_sa.is_ipv4());

    //constructs a SocketAddr from IPv6 address and a port, assert that
    // the underlying address is indeed IPv6
    let global_sa = SocketAddr::new(global, 80u16);
    assert!(global_sa.is_ipv6());

    
}