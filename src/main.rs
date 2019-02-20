//! Simple TLS parser.
//!
//! This Rust crate is a proof-of-concept implementation of a TLS certificate validator. As a PoC
//! currently we only use pcap packet traces as input. The current progress is tracked in `examples/` and
//! the active developing part is in `src/`.
//!
//! # Examples (missing)
//! ```sh
//! # this command will parse tls-all.pcap file and give the output
//! $ cargo run --example parser-all
//! # this command will parse tls-cert.pcap file and give the output
//! $ cargo run --example parser-cert
//! ```
//! # TODO
//! * handle tcp segment reassemble so that we can retrieve the certificate.
extern crate clap;
extern crate env_logger;
extern crate failure;
extern crate futures;
//extern crate h2;
extern crate pcap;
extern crate rustls;
extern crate smoltcp;

use std::path::Path;

//use clap::{App, Arg};
use failure::{err_msg, Error};
//use futures::prelude::*;
//use h2::Codec;
use pcap::Capture;
use rustls::internal::msgs::{
    codec::Codec, enums::ContentType, message::Message as TLSMessage, message::MessagePayload,
};
use rustls::internal::msgs::{
    enums::ServerNameType, handshake::ClientHelloPayload, handshake::HandshakePayload,
    handshake::HasServerExtensions, handshake::ServerHelloPayload, handshake::ServerNamePayload,
};

//use rustls::{CipherSuite, ProtocolVersion};
use smoltcp::wire::*;
//use std::net::Ipv4Addr;

/// Endpoint parsing function.
///
/// The function will take a simple endpoint string and parse it into ip address and port number as
/// we desired.
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

/// Pcap file parser.
///
/// The function will take a pcap file as input and parse it for TLS handshake messages.
fn dump_file<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let mut counter = 1;
    let mut cap = Capture::from_file(path)?;

    // define a bogus client side ip addr
    let client_endpoint = parse_endpoint("10.200.205.238:59295")?;
    // server side ip addr: google
    //let server_endpoint = parse_endpoint("192.30.253.117:443")?;

    while let Ok(packet) = cap.next() {
        //println!("This is the {}th packet.", counter); // sanity check

        let ether = EthernetFrame::new_checked(packet.data).map_err(err_msg)?;
        if EthernetProtocol::Ipv4 == ether.ethertype() {
            let ipv4 = Ipv4Packet::new_checked(ether.payload()).map_err(err_msg)?;

            // if packet goes to client
            if IpAddress::from(ipv4.dst_addr()) == client_endpoint.addr {
                let tcp = TcpPacket::new_checked(ipv4.payload()).map_err(err_msg)?;
                let _seq_num = tcp.seq_number();
                let _ack_num = tcp.ack_number();
                let _fin = tcp.fin();

                if tcp.dst_port() == client_endpoint.port {
                    println!("This is a packet for the client!!!!");
                    //println!("Payload is: {:x?}", tcp.payload());

                    let pkt = TLSMessage::read_bytes(&tcp.payload());
                    //println!("{:?}", packet);
                    println!(
                        "PACKET: {} --- Seq No: {}, ACK No: {}, FIN Flag: {}",
                        counter, _seq_num, _ack_num, _fin
                    );

                    match pkt {
                        Some(mut packet) => {
                            println!("Type of the packet is: {:?}", packet.typ);
                            // TODO: need to reassemble tcp segements
                            if packet.typ == ContentType::Handshake && packet.decode_payload() {
                                if let MessagePayload::Handshake(x) = packet.payload {
                                    println!(
                                        "Handshake is {:?}, packet version is {:?}",
                                        x, packet.version
                                    );
                                } else {
                                    println!("Packet payload type doesn't match handshake!");
                                    println!("But we want to print it anyway {:?}", packet.payload);
                                }
                            } else {
                                println!("Packet payload type doesn't match handshake!");
                                println!("But we want to print it anyway {:?}", packet.payload);
                            }
                        }
                        None => println!("There is no packet"),
                    }
                }
            }
            counter = counter + 1;
        }
    }
    Ok(())
}

fn main() {
    env_logger::init();

    //let input_file = "data/tls-cert.pcap";
    let input_file = "data/tls-all.pcap";

    if let Err(err) = dump_file(input_file) {
        eprintln!("error: failed to dump pcap file");
        for cause in err.iter_chain() {
            eprintln!("caused by: {}", cause);
        }
        for line in err.backtrace().to_string().lines() {
            eprintln!("{}", line);
        }
        std::process::exit(1);
    }
}
