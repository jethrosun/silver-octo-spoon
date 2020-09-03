use failure::Fallible;
use serde_json::{from_reader, Result, Value};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::vec::Vec;

fn rdr_read_rand_seed(file_path: String, setup: String) {
    let file = File::open(file_path).expect("file should open read only");
    let json_data: Value = from_reader(file).expect("file should be proper JSON");
    // println!("{:?}", json_data);

    let rdr_data = match json_data.get("rdr") {
        Some(val) => {
            let rdr_rand_seeds = match json_data.get(setup) {
                Some(val) => {
                    println!("val: {:?}", val);
                
                None => println!("nothing"),
            };
        }
        None => println!("nothing"),
    };
}

fn p2p_read_rand_seed(file_path: String, setup: usize) {}

fn main() {
    // Workloads:

    let workload_path = "/home/jethros/dev/pvn/utils/rand_number/rand.json";

    let mut rdr_rand_seed = rdr_read_rand_seed(workload_path.to_string(), "setup-5".to_string());
}
