use crossbeam::thread;
use serde_json::{from_reader, Value};
use std::collections::{HashMap, HashSet};
use std::fs;
use transmission::{Client, ClientConfig};

/// Get the parameters for running p2p experiments.
///
/// 1 torrent job in total -- 3% pktgen sending rate
/// 5 torrent job in total -- 13% pktgen sending rate
/// 10 torrent job in total -- 25% pktgen sending rate
/// 20 torrent job in total -- 50% pktgen sending rate
/// 30 torrent job in total -- 75% pktgen sending rate
/// 40 torrent job in total -- 100% pktgen sending rate
pub fn p2p_retrieve_param(setup_val: usize) -> Option<usize> {
    let mut map = HashMap::new();
    map.insert(1, 1);
    map.insert(2, 5);
    map.insert(3, 10);
    map.insert(4, 20);
    map.insert(5, 30);
    map.insert(6, 40);

    map.remove(&setup_val)
}

pub fn load_json(file_path: String) -> Vec<String> {
    let file = fs::File::open(file_path).expect("file should open read only");
    let json: Value = from_reader(file).expect("file should be proper JSON");

    let torrent_files = json
        .get("torrents_files")
        .expect("file should have time key")
        .clone();
    // println!("\ntorrent_files {:?}", torrent_files);

    let torrents: Vec<String> = serde_json::from_value(torrent_files).unwrap();
    // println!("\ntorrents {:?}", torrents);
    torrents
}

pub fn run_torrent_test(pivot: u64, workload: &mut Vec<String>, torrents_dir: &str, c: &Client) {
    // println!("run torrents {:?}", pivot);
    match workload.pop() {
        Some(torrent) => {
            println!("{:?} torrent is : {:?}", pivot, torrent);
            let torrent = torrents_dir.clone().to_owned() + &torrent;
            // println!("torrent dir is : {:?}", torrent_dir);
            let t = c.add_torrent_file(&torrent).unwrap();
            t.start();
        }
        None => {
            println!("no torrent");
        }
    }
}

pub fn task_scheduler(
    pivot: u64,
    c: &Client,
    workload: &mut Vec<String>,
    torrents_dir: &str,
    config_dir: &str,
    download_dir: &str,
) {
    // println!("run torrents {:?}", pivot);
    match workload.pop() {
        Some(torrent) => {
            println!("{:?} torrent is : {:?}", pivot, torrent);
            let torrent = torrents_dir.clone().to_owned() + &torrent;
            // println!("torrent dir is : {:?}", torrent_dir);
            run_torrent(
                c,
                &torrent,
                &config_dir.to_string(),
                &download_dir.to_string(),
            );
        }
        None => {
            println!("no torrent");
        }
    }
}

pub fn run_torrent(c: &Client, torrent: &str, config_dir: &str, download_dir: &str) {
    thread::scope(|s| {
        let core_ids = core_affinity::get_core_ids().unwrap();
        let handles = core_ids
            .into_iter()
            .map(|id| {
                s.spawn(move |_| {
                    // Pin this thread to a single CPU core.
                    core_affinity::set_for_current(id);
                    // Do more work after this.
                    //
                    if id.id == 5 as usize {
                        println!("Working in core {:?}", id);
                        let t = c.add_torrent_file(torrent).unwrap();
                        t.start();
                    }
                })
            })
            .collect::<Vec<_>>();

        for handle in handles.into_iter() {
            handle.join().unwrap();
        }
    })
    .unwrap();
}
