use rand::{distributions::Uniform, Rng};
use serde_json::{from_reader, from_value, Value};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration; // 0.6.5

// pub fn load_json(file_path: String) -> Result<()> {
pub fn load_json(file_path: String) {
    let file = File::open(file_path).expect("file should open read only");
    let json: Value = from_reader(file).expect("file should be proper JSON");

    let time_value = json.get("time").expect("file should have time key").clone();
    let user_num_value = json
        .get("number_of_user")
        .expect("file should have number_of_user key")
        .clone();
    let total_visited_times_value = json
        .get("total_visited_times")
        .expect("file should have time key")
        .clone();
    let urls_value = json
        .get("urls")
        .expect("file should have number_of_user key")
        .clone();
    let visited_times_value = json
        .get("visited_times")
        .expect("file should have number_of_user key")
        .clone();

    let time: usize = serde_json::from_value(time_value).unwrap();
    println!("time: {}", time);
    let user_num: usize = serde_json::from_value(user_num_value).unwrap();
    println!("user_num: {}", user_num);
    let total_visited_times: usize = serde_json::from_value(total_visited_times_value).unwrap();
    println!("total visited time: {}", time);
    let urls: Vec<String> = serde_json::from_value(urls_value).unwrap();
    println!("urls: {:?}", urls);
    let visited_times: Vec<u64> = serde_json::from_value(visited_times_value).unwrap();
    println!("visited_times: {:?}", visited_times);

    create_workload(time, total_visited_times, urls, visited_times)
}

fn create_workload(
    time: usize,
    total_visited_times: usize,
    urls: Vec<String>,
    visited_times: Vec<u64>,
) {
    let bucket_size = time * 6;
    let mut workload: Vec<Vec<String>> = Vec::new();

    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, bucket_size as u64);

    let index_list: Vec<u64> = (0..total_visited_times)
        .map(|_| rng.sample(&range))
        .collect();
    let mut iter = index_list.iter();

    // for n in 0..=urls.len() {
    //     for i in 0..=visited_times[n].len() {
    //         workload[iter.next().unwrap()].push(urls[n]);
    //     }
    //     println!("{}", n);
    // }

    unimplemented!();
}
