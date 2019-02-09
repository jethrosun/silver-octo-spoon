extern crate etherparse;

use etherparse::SlicedPacket;
use pcap_file::{PcapReader, PcapWriter};

use std::fs::File;

use std::net::Ipv4Addr;

fn main() {
    println!("Hello, world!");
    let file_in = File::open("data/tls-real.pcap").expect("Error opening file");
    let pcap_reader = PcapReader::new(file_in).unwrap();

    //let file_out = File::create("data/output/out.pcap").expect("Error creating file");
    //let mut pcap_writer = PcapWriter::new(file_out).unwrap();
    let mut counter = 0;
    // Read test.pcap
    for pcap in pcap_reader {
        //Check if there is no error
        let pkt = pcap.unwrap();

        // We know in the pcap file, the 4th, 5th, 6th packets give us the certificate
        if counter > 3 {
            if counter < 7 {
                println!("This is the {}th packet", counter);
                println!("{:?}", pkt);

                match SlicedPacket::from_ethernet(&pkt) {
                    Err(value) => println!("Err {:?}", value),
                    Ok(value) => {
                        println!("link: {:?}", value.link);
                        println!("vlan: {:?}", value.vlan);
                        println!("ip: {:?}", value.ip);
                        println!("transport: {:?}", value.transport);
                    }
                }
            }
        }

        counter = counter + 1;
    }
    println!("There are {} number of packets!", counter);
}
