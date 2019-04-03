use failure::{err_msg, Error};
//use pcap::Capture;
use rustls::internal::msgs::message::Message as TLSMessage;
use smoltcp::wire::*;
use std::io;
//use std::path::Path;

#[derive(Debug)]
pub struct Flow {
    ip_endpoint: IpEndpoint,
    flow_content: Vec<TLSMessage>,
}

impl Flow {
    pub fn new(ip_end_point: IpEndpoint, pkt: TLSMessage) -> (Self) {
        let mut vec: Vec<TLSMessage> = Vec::new();
        vec.push(pkt);
        Flow {
            ip_endpoint: ip_end_point,
            flow_content: vec,
        }
    }
    pub fn push(mut self, pkt: TLSMessage) -> (Flow) {
        self.flow_content.push(pkt);
        self
    }
}

/// Insert a packet into a flow.
///
/// The key will be the IpEndpoint and the hash value will be ?
pub fn insert_flow_cache(flow: Flow, pkt: TLSMessage) -> (Flow) {
    flow.push(pkt)
}

/// The current packet belongs to a flow and the flow
///
pub fn dump_flow(flow: Flow) -> (io::Result<usize>) {
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
