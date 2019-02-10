extern crate clap;
extern crate env_logger;
extern crate failure;
extern crate futures;
extern crate h2;
extern crate pcap;
extern crate rustls;
extern crate smoltcp;
extern crate tokio_io;

use std::path::Path;

use clap::{App, Arg};
use failure::{err_msg, Error};
use futures::prelude::*;
//use h2::Codec;
use pcap::Capture;
use smoltcp::wire::*;
use tokio_io::io::read_exact;

use rustls::internal::msgs::{
    codec::Codec, enums::ContentType, enums::ServerNameType, handshake::ClientHelloPayload,
    handshake::HandshakePayload, handshake::HasServerExtensions, handshake::ServerHelloPayload,
    handshake::ServerNamePayload, message::Message as TLSMessage, message::MessagePayload,
};
use rustls::{CipherSuite, ProtocolVersion};

use std::net::Ipv4Addr;

fn parse_endpoint(endpoint: &str) -> Result<IpEndpoint, Error> {
    let mut iter = endpoint.rsplitn(2, ':');
    let port = iter.next().ok_or(err_msg("missing port"))?.parse::<u16>()?;
    let addr = iter
        .next()
        .ok_or(err_msg("missing address"))?
        .parse::<IpAddress>()
        .map_err(|_| err_msg("failed to parse ip address"))?;
    Ok(IpEndpoint::new(addr, port))
}

fn dump_file<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let mut counter = 1;
    let mut cap = Capture::from_file(path)?;

    let src_endpoint = parse_endpoint("192.30.253.117:10")?;
    let dst_endpoint = parse_endpoint("10.200.205.238:59295")?;

    while let Ok(pkt) = cap.next() {
        //println!("{:?}", packet); // sanity check
        let mut version = ProtocolVersion::Unknown(0x0000);
        //let (handshake, version) = {
        let mut packet =
            TLSMessage::read_bytes(&pkt()).ok_or(err_msg("packet read byte failed"))?;

        if packet.typ == ContentType::Handshake && packet.decode_payload() {
            if let MessagePayload::Handshake(x) = packet.payload {
                println!("{:?}, {:?}", x, packet.version);
            //Ok(())
            } else {
                //Ok(())
                //return None;
            }
        } else {
            //Ok(())
            //return None;
        }
    }
    Ok(())
}

fn main() {
    env_logger::init();

    println!("{:?}", ContentType::Handshake);
    let input_file = "data/tls-all.pcap";

    if let Err(err) = dump_file(input_file) {
        eprintln!("error: failed to dump pcap file");
        for cause in err.causes() {
            eprintln!("caused by: {}", cause);
        }

        for line in err.backtrace().to_string().lines() {
            eprintln!("{}", line);
        }

        std::process::exit(1);
    }
}
