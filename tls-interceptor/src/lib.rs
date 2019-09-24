//! Simple TLS parser.
//!
//! This Rust crate is a proof-of-concept implementation of a TLS certificate validator. As a PoC
//! currently we only use pcap packet traces as input. The current progress is tracked in `examples/` and
//! the active developing part is in `src/`.
//!
//! # Examples (missing)
//! ```sh
//! # running the last version
//! $ cargo run --example last
//! # this command will parse tls-all.pcap file and give the output
//! $ cargo run --example parser-all
//! # this command will parse tls-cert.pcap file and give the output
//! $ cargo run --example parser-cert
//! ```
//! # TODO
//! * handle tcp segment reassemble so that we can retrieve the certificate.
//!
use failure::{err_msg, Error};
//use pcap::Capture;
use smoltcp::wire::*;
use std::io;
//use std::path::Path;

#[derive(Debug)]
pub struct Flow<T>
where
    T: std::convert::AsRef<[u8]>,
{
    ip_endpoint: IpEndpoint,
    flow_content: Vec<TcpPacket<T>>,
}

// https://users.rust-lang.org/t/how-do-i-do-insert-update-of-a-vec-inside-a-hashmap/17092
impl<T> Flow<T>
where
    T: std::convert::AsRef<[u8]>,
{
    pub fn new(ip_end_point: IpEndpoint, _pkt: TcpPacket<T>) -> (Self) {
        let mut vec: Vec<TcpPacket<T>> = Vec::new();
        vec.push(_pkt);
        Flow {
            ip_endpoint: ip_end_point,
            flow_content: vec,
        }
    }

    pub fn insert_pkt(&mut self, _pkt: TcpPacket<T>) {
        self.flow_content.push(_pkt);
    }
}

/// Insert a packet into a flow.
///
/// The key will be the IpEndpoint and the hash value will be ?
// pub fn insert_flow_cache<T>(flow: Flow<T>, pkt: TcpPacket<T>) -> (Flow<T>)
// where
//     T: std::convert::AsRef<[u8]>,
// {
//     //flow.push(pkt)
// }

/// The current packet belongs to a flow and the flow
///
pub fn dump_flow<T>(_flow: Flow<T>) -> (io::Result<usize>)
where
    T: std::convert::AsRef<[u8]>,
{
    unimplemented!();
}

/// Endpoint parsing function.
///
/// The function will take a simple endpoint string and parse it into ip address and port number as
/// we desired.
pub fn parse_endpoint(endpoint: &str) -> Result<IpEndpoint, Error> {
    let mut iter = endpoint.rsplitn(2, ':');
    //let port = iter.next().ok_or(err_msg("missing port"))?.parse::<u16>()?;
    let port = iter
        .next()
        .ok_or_else(|| err_msg("missing port"))?
        .parse::<u16>()?;
    let addr = iter
        .next()
        .ok_or_else(|| err_msg("missing address"))?
        .parse::<IpAddress>()
        .map_err(|_| err_msg("failed to parse ip address"))?;
    Ok(IpEndpoint::new(addr, port))
}
