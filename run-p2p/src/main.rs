use lib::*;
use std::collections::HashMap;
use std::convert::From;
use std::hash::BuildHasherDefault;
use std::net::Ipv4Addr;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use transmission::{Client, ClientConfig};

mod lib;

fn main() {
    let mut pivot = 0;

    // Workload and States for P2P NF
    //
    // 1, 10, 20, 40, 50, 75, 100, 150, 200
    let workload = "/home/jethros/dev/pvn-utils/workload/p2p-workload.json";
    println!("{:?}", workload);
    let mut workload = load_json(workload.to_string());

    let mut torrent_list = Vec::new();

    // Fixed transmission setup
    let torrents_dir = "/home/jethros/dev/pvn-utils/workload/torrent_files/";

    let config_dir = "/data/config";
    let download_dir = "/data/downloads";

    let config = ClientConfig::new()
        .app_name("testing")
        .config_dir(config_dir)
        .use_utp(false)
        .download_dir(download_dir);
    let c = Client::new(config);

    while let Some(torrent) = workload.pop() {
        if pivot >= 5 {
            break;
        }
        println!("torrent is : {:?}", torrent);
        let torrent = torrents_dir.to_owned() + &torrent;
        // println!("torrent dir is : {:?}", torrent_dir);
        let t = c.add_torrent_file(&torrent).unwrap();
        t.start();
        torrent_list.push(t);
        pivot += 1;
    }

    loop {
        if torrent_list.clone().into_iter().all(|x| x.stats().finished) {
            println!("All done");
        }
    }
}
