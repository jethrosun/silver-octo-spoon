use failure::Fallible;
use serde_json::{from_reader, Result, Value};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::vec::Vec;

fn rdr_read_rand_seed(file_path: String, setup: String, iter: String) -> Result<Vec<i64>> {
    let mut rand_vec = Vec::new();
    let file = File::open(file_path).expect("rand seed file should open read only");
    let json_data: Value = from_reader(file).expect("file should be proper JSON");

    match json_data.get("rdr") {
        Some(rdr_data) => match rdr_data.get(setup.clone()) {
            Some(setup_data) => match setup_data.get(iter.clone()) {
                Some(data) => {
                    for x in data.as_array().unwrap() {
                        rand_vec.push(x.as_i64().unwrap());
                        println!("{:?}", x.as_i64().unwrap());
                    }
                }
                None => println!("No rand data for iter {:?} in setup {:?}", iter, setup),
            },
            None => println!("No rand data for setup {:?}", setup),
        },
        None => println!("No rdr data in the rand seed file"),
    }
    Ok(rand_vec)
}

fn p2p_read_rand_seed(file_path: String, setup: String, iter: String) -> Result<Vec<i64>> {
    let mut rand_vec = Vec::new();
    let file = File::open(file_path).expect("file should open read only");
    let json_data: Value = from_reader(file).expect("file should be proper JSON");

    match json_data.get("p2p") {
        Some(p2p_data) => match p2p_data.get(setup.clone()) {
            Some(setup_data) => match setup_data.get(iter.clone()) {
                Some(data) => {
                    for x in data.as_array().unwrap() {
                        rand_vec.push(x.as_i64().unwrap());
                    }
                }
                None => println!("No rand data for iter {:?} in setup {:?}", iter, setup),
            },
            None => println!("No rand data for setup {:?}", setup),
        },
        None => println!("No p2p data in the rand seed file"),
    }
    Ok(rand_vec)
}

fn main() {
    // Workloads:

    let workload_path = "/home/jethros/dev/pvn/utils/rand_number/rand.json";

    let mut rdr_rand_seed = rdr_read_rand_seed(
        workload_path.to_string(),
        "setup-5".to_string(),
        0.to_string(),
    );
}
