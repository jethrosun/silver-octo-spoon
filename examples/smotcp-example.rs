//! This is my awesome crate
//!
//! Here goes some other description of what it is and what is does
//!
//! # Examples
//! ```
//! fn sum2(n1: i32, n2: i32) -> i32 {
//!   n1 + n2
//! }
//! # assert_eq!(4, sum2(2, 2));
//! ```

extern crate etherparse;

use etherparse::SlicedPacket;
use pcap_file::{PcapReader, PcapWriter};

use std::fs::File;

use std::net::Ipv4Addr;

use smoltcp::phy::ChecksumCapabilities;
use smoltcp::wire::*;

fn main() {
    println!("Hello, world!");
    let repr = Ipv4Repr {
        src_addr: Ipv4Address::new(10, 0, 0, 1),
        dst_addr: Ipv4Address::new(10, 0, 0, 2),
        protocol: IpProtocol::Tcp,
        payload_len: 10,
        hop_limit: 64,
    };
    let mut buffer = vec![0; repr.buffer_len() + repr.payload_len];
    {
        // emission
        let mut packet = Ipv4Packet::new_unchecked(&mut buffer);
        repr.emit(&mut packet, &ChecksumCapabilities::default());
    }
    {
        // parsing
        let packet = Ipv4Packet::new_checked(&buffer).expect("truncated packet");
        let parsed =
            Ipv4Repr::parse(&packet, &ChecksumCapabilities::default()).expect("malformed packet");
        assert_eq!(repr, parsed);

        println!("{:?}", packet);
    }
}
