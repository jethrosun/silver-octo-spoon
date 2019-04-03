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
use lib::Flow;
use lib::{insert_flow_cache, parse_endpoint};
use pcap::Capture;
use rustls::internal::msgs::{
    codec::Codec,
    enums::ContentType,
    message::Message as TLSMessage, //message::MessagePayload,
};
use smoltcp::wire::*;
use std::collections::HashMap;
use std::path::Path;
use std::vec::Vec;

mod lib;

/// Pcap file parser.
///
/// The function will take a pcap file as input and parse it for TLS handshake messages.
fn dump_file<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    //let mut flows = HashMap::new();
    let mut counter = 1;
    let mut cap = Capture::from_file(path)?;
    let mut flow_group: HashMap<IpEndpoint, Flow<&[u8]>> = HashMap::new();

    // define a bogus client side ip addr
    let client_endpoint = parse_endpoint("10.200.205.238:59295")?;
    println!("{}", client_endpoint);
    // server side ip addr: google
    //let server_endpoint = parse_endpoint("192.30.253.117:443")?;
    let mut expected_seq_no = TcpSeqNumber(0);

    while let Ok(packet) = cap.next() {
        // println!(
        //     "\nThis is the {}th packet, and we are expecting {}",
        //     counter, expected_seq_no
        // ); // sanity check
        // ethernet packet
        let ether = EthernetFrame::new_checked(packet.data).map_err(err_msg)?;
        if EthernetProtocol::Ipv4 == ether.ethertype() {
            let ipv4 = Ipv4Packet::new_checked(ether.payload()).map_err(err_msg)?;

            // filter: if packet goes to client
            if IpAddress::from(ipv4.dst_addr()) == client_endpoint.addr {
                let tcp_pkt = TcpPacket::new_unchecked(ipv4.payload());
                let _seq_num = tcp_pkt.seq_number();
                let _ack_num = tcp_pkt.ack_number();
                let _fin = tcp_pkt.fin();
                let _psh = tcp_pkt.psh();
                let _seg_len = tcp_pkt.segment_len();

                if tcp_pkt.dst_port() == client_endpoint.port {
                    println!();
                    println!(
                        "PACKET: {} --- Seq No: {}, ACK No: {}, FIN Flag: {}, PSH Flag: {}",
                        counter, _seq_num, _ack_num, _fin, _psh
                    );

                    if _seq_num == expected_seq_no {
                        println!("We are expecting this exact packet!!!");
                    } else {
                        println!("This is not the packet we are looking for");
                    }

                    println!("This is a packet for the client!!!!");
                    //println!("Payload is: {:x?}", tcp.payload());

                    // parse packet into TCP header + TCP payload
                    let pkt = TLSMessage::read_bytes(&tcp_pkt.payload());
                    //println!("Read tcp packet payload as \n {:?}", packet);

                    expected_seq_no = match pkt {
                        Some(packet) => {
                            println!("Type of the packet is: {:?}", packet.typ);
                            // TODO: need to reassemble tcp segements
                            if packet.typ == ContentType::Handshake && !_psh {
                                println!("Packet is a TLS handshake but it is not yet complete, we now insert the current packet into the flow cache!");

                                //let mut current_flow = Flow::new(client_endpoint, packet);
                                //flow_group.insert(client_endpoint, current_flow);
                                //println!("{:?}", flow_group);

                                //let flow = insert_flow_cache(Some(&client_endpoint), tcp_pkt);
                                _seq_num + _seg_len
                            } else {
                                println!("Orphan packet!!");
                                _seq_num + _seg_len
                            }
                        }
                        None => {
                            // matched none, very likely this a segmented packet
                            println!("==========Matched NONE============");
                            dbg!(pkt);
                            if _psh {
                                println!("Push flag is true. We should dump the whole flow!!");

                                match flow_group.get(&client_endpoint) {
                                    Some(flow) => println!("Calling Ashley:"),
                                    _ => println!("Don't have Ashley's number."),
                                }
                                //insert_flow_cache(current_flow, packet);

                                //let string = insert_flow_cache(Some(&client_endpoint), tcp_pkt);
                                _seq_num + _seg_len
                            } else {
                                println!(
                                    "Packet should be a segmented packet in the middle of a flow!"
                                );
                                // be careful about the implemnetation: https://users.rust-lang.org/t/how-do-i-do-insert-update-of-a-vec-inside-a-hashmap/17092/2
                                //flow_group.entry(&client_endpoint).push(packet);
                                match flow_group.get_mut(&client_endpoint) {
                                    //Some(flow) => flow.push(pkt),
                                    Some(flow) => println!("Figured out ,,,"),
                                    None => {
                                        println!("Figured out ,,,")
                                        //Flow::new(client_endpoint, pkt)
                                    }
                                }

                                _seq_num + _seg_len
                            }
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
