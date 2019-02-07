#![feature(try_from)]

use pcap_file::{PcapReader, PcapWriter};
use std::fs::File;

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
        let pcap = pcap.unwrap();
        //println!("{}", pcap);

        counter = counter + 1;
        //Write each packet of test.pcap in out.pcap
        //pcap_writer.write_packet(&pcap).unwrap();
        //
    }
    println!("There are {} number of packets!", counter);
}
