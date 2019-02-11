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
use rustls::internal::msgs::{
    codec::Codec, enums::ContentType, enums::ServerNameType, handshake::ClientHelloPayload,
    handshake::HandshakePayload, handshake::HasServerExtensions, handshake::ServerHelloPayload,
    handshake::ServerNamePayload, message::Message as TLSMessage, message::MessagePayload,
};
use rustls::{CipherSuite, ProtocolVersion};
use smoltcp::wire::*;
use std::net::Ipv4Addr;
use tokio_io::io::read_exact;

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

    // define a bogus client side ip addr
    let client_endpoint = parse_endpoint("10.200.205.238:59295")?;
    // server side ip addr: google
    let server_endpoint = parse_endpoint("192.30.253.117:443")?;

    while let Ok(packet) = cap.next() {
        //println!("{:?}", packet); // sanity check

        let ether = EthernetFrame::new_checked(packet.data).map_err(err_msg)?;
        if EthernetProtocol::Ipv4 == ether.ethertype() {
            let ipv4 = Ipv4Packet::new_checked(ether.payload()).map_err(err_msg)?;

            // if packet goes to client
            if IpAddress::from(ipv4.dst_addr()) == client_endpoint.addr {
                let tcp = TcpPacket::new_checked(ipv4.payload()).map_err(err_msg)?;
                if tcp.dst_port() == client_endpoint.port {
                    println!("This is a packet for the client!!!!");
                    //println!("Payload is: {:x?}", tcp.payload());

                    let mut packet = TLSMessage::read_bytes(&tcp.payload_mut())?; //.ok_or("Failed")?;
                                                                                  //println!("{:?}", packet);
                    if packet.typ == ContentType::Handshake && packet.decode_payload() {
                        if let MessagePayload::Handshake(x) = packet.payload {
                            println!("Handshake is {}, packet version is {}", x, packet.version);
                        } else {
                            println!("Packet payload doesnot match handshake!");
                        }
                    } else {
                        println!("Packet type is not matched!")
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
        for cause in err.causes() {
            eprintln!("caused by: {}", cause);
        }

        for line in err.backtrace().to_string().lines() {
            eprintln!("{}", line);
        }

        std::process::exit(1);
    }
}
