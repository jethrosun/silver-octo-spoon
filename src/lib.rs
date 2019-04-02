use failure::{err_msg, Error};
use pcap::Capture;
use rustls::internal::msgs::{
    codec::Codec,
    enums::ContentType,
    message::Message as TLSMessage, //message::MessagePayload,
};
use smoltcp::wire::*;
use std::io;
use std::path::Path;

/// Insert a packet into a flow.
///
/// The key will be the IpEndpoint and the hash value will be ?
pub fn insert_flow_cache<T>(endpoint: Option<&IpEndpoint>, _pkt: TcpPacket<T>) -> &str
where
    T: std::convert::AsRef<[u8]>,
    T: std::fmt::Debug,
{
    match endpoint {
        Some(ip_endpoint) => {
            println!("{:?}", ip_endpoint);
            "Some insert flow cache!"
        }
        None => "Not recognized from insert flow cache!",
    }
}

/// The current packet belongs to a flow and the flow
///
pub fn dump_flow<T>(endpoint: Option<&IpEndpoint>, _pkt: TcpPacket<T>) -> (&str, io::Result<u8>)
where
    T: std::convert::AsRef<[u8]>,
    T: std::fmt::Debug,
{
    match endpoint {
        Some(ip_endpoint) => {
            println!("{:?}", ip_endpoint);
            ("Some insert flow cache!", unimplemented!())
        }
        None => ("Not recognized from insert flow cache!", unimplemented!()),
    }
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
