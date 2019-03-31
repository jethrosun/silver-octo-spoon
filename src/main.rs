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
extern crate pcap;
extern crate rustls;
extern crate smoltcp;

use failure::{err_msg, Error};
use pcap::Capture;
use rustls::internal::msgs::{
    codec::Codec,
    enums::ContentType,
    message::Message as TLSMessage, //message::MessagePayload,
};
use smoltcp::wire::*;
use std::path::Path;
use utils::{insert_flow_cache, parse_endpoint};

mod utils;

/// Pcap file parser.
///
/// The function will take a pcap file as input and parse it for TLS handshake messages.
fn dump_file<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    //let mut flows = HashMap::new();
    let mut counter = 1;
    let mut cap = Capture::from_file(path)?;

    // define a bogus client side ip addr
    let client_endpoint = parse_endpoint("10.200.205.238:59295")?;
    println!("{}", client_endpoint);
    // server side ip addr: google
    //let server_endpoint = parse_endpoint("192.30.253.117:443")?;

    while let Ok(packet) = cap.next() {
        //println!("This is the {}th packet.", counter); // sanity check

        let ether = EthernetFrame::new_checked(packet.data).map_err(err_msg)?;
        if EthernetProtocol::Ipv4 == ether.ethertype() {
            let ipv4 = Ipv4Packet::new_checked(ether.payload()).map_err(err_msg)?;

            // if packet goes to client
            if IpAddress::from(ipv4.dst_addr()) == client_endpoint.addr {
                let tcp_pkt = TcpPacket::new_checked(ipv4.payload()).map_err(err_msg)?;
                let _seq_num = tcp_pkt.seq_number();
                let _ack_num = tcp_pkt.ack_number();
                let _fin = tcp_pkt.fin();
                let _psh = tcp_pkt.psh();

                if tcp_pkt.dst_port() == client_endpoint.port {
                    println!();
                    println!(
                        "PACKET: {} --- Seq No: {}, ACK No: {}, FIN Flag: {} PSH Flag P{}",
                        counter, _seq_num, _ack_num, _fin, _psh
                    );

                    println!("This is a packet for the client!!!!");
                    //println!("Payload is: {:x?}", tcp.payload());

                    let pkt = TLSMessage::read_bytes(&tcp_pkt.payload());
                    //println!("{:?}", packet);

                    match pkt {
                        Some(packet) => {
                            println!("Type of the packet is: {:?}", packet.typ);
                            // TODO: need to reassemble tcp segements
                            if packet.typ == ContentType::Handshake && !_psh {
                                println!("Packet is a TLS handshake but it is not yet complete, we now insert the current packet into the flow cache!");
                                let string = insert_flow_cache(&client_endpoint, tcp_pkt);
                                println!("{}", string);
                            } else {
                                println!("Packet is a TLS handshake!");
                            }
                        }
                        None => {
                            //println!();
                            println!("DEBUG: matched pkt with None");
                            // I should concat this to the previous packet
                            //println!("Previous packet {:?}", prev_packet);
                            println!("So we just print the bytes we have {:?}", pkt);
                        }
                    }
                }
            }
            counter += 1;
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
        for _line in err.backtrace().to_string().lines() {}
        std::process::exit(1);
    }
}
