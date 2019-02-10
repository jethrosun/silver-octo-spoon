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
use rustls::internal::msgs::enums::ContentType;
use smoltcp::wire::*;
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

    let src_endpoint = parse_endpoint("192.30.253.117:10")?;
    let dst_endpoint = parse_endpoint("10.200.205.238:59295")?;

    while let Ok(packet) = cap.next() {
        //println!("{:?}", packet); // sanity check

        let ether = EthernetFrame::new_checked(packet.data).map_err(err_msg)?;
        if EthernetProtocol::Ipv4 == ether.ethertype() {
            let ipv4 = Ipv4Packet::new_checked(ether.payload()).map_err(err_msg)?;

            if IpAddress::from(ipv4.src_addr()) == src_endpoint.addr {
                if IpAddress::from(ipv4.dst_addr()) == dst_endpoint.addr {
                    let tcp = TcpPacket::new_checked(ipv4.payload()).map_err(err_msg)?;
                    if tcp.dst_port() == dst_endpoint.port {
                        //client.push(tcp.payload());
                        println!("Matched!!!!");
                        println!("{:?}", tcp.payload());

                        ;
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
