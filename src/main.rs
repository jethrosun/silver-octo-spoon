//#![cfg_attr(feature = "nightly", feature(nll))]
//#![feature(try_from)]

extern crate clap;
extern crate env_logger;
extern crate failure;
extern crate futures;
extern crate net_parser_rs;
extern crate pcap;
extern crate rustls;
extern crate smoltcp;

use failure::{err_msg, Error};
use lib::parse_endpoint;
use lib::Flow;
use net_parser_rs::flow::*;
use net_parser_rs::CaptureParser;
use pcap::{Capture, Packet};
use smoltcp::wire::*;
use std::collections::HashMap;
use std::path::Path;
use std::*;

mod lib;

/// Pcap file parser.
///
/// The function will take a pcap file as input and parse it for TLS handshake messages.
fn dump_file<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let mut cap = Capture::from_file(path)?;

    //let mut flows = HashMap::new();
    let mut counter = 1;
    let mut flow_group: Vec<Flow<&[u8]>> = Vec::new();

    // define a bogus client side ip addr
    let client_endpoint = parse_endpoint("10.200.205.238:59295")?;
    println!("{}", client_endpoint);
    // server side ip addr: google
    //let server_endpoint = parse_endpoint("192.30.253.117:443")?;
    let mut expected_seq_no = TcpSeqNumber(0);
    let mut expected =
    //let mut packet: Result<Packet, Error>;

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

                    // Beginning of the parsing

                    let flow = if !_psh {
                        expected_seq_no = _seq_num + _seg_len;
                        println!(
                            "Packet has no Push flag, which means it is a fraction of something!"
                        );
                        Flow::new(client_endpoint, tcp_pkt)
                    // println!("{:?}", flow_group);
                    } else if _seq_num == expected_seq_no {
                        expected_seq_no = _seq_num + _seg_len;
                        println!("Seq number equals to our expected seq number");
                        Flow::new(client_endpoint, tcp_pkt)
                    } else {
                        expected_seq_no = _seq_num + _seg_len;
                        println!("Seq number doesn't equal to expected seq number, this shouldn't have happened.");
                        Flow::new(client_endpoint, tcp_pkt)
                    };
                    flow_group.push(flow);
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
