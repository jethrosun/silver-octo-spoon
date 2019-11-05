extern crate rustc_serialize;
use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;

mod lib;

fn main() {
    let mut file = File::open("workload1.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let json = Json::from_str(&data).unwrap();
    println!("{}", json.find_path(&["Address", "Street"]).unwrap());
}
