use rand::Rng;
use serde_json::json;
use serde_json::{from_reader, Result, Value};
use std::collections::HashMap;
use std::fs::File;

fn get_magic_num(range: usize) -> usize {
    let num = rand::thread_rng().gen_range(0, range);
    num
}

/// Construct the workload from the session file.
///
/// https://kbknapp.github.io/doapi-rs/docs/serde/json/index.html
pub fn rdr_load_workload(
    file_path: String,
    num_of_secs: usize,
    num_of_user: usize,
) -> Result<HashMap<usize, HashMap<usize, Vec<(i32, String)>>>> {
    // time in second, workload in that second
    let mut workload =
        HashMap::<usize, HashMap<usize, Vec<(i32, String)>>>::with_capacity(num_of_secs);

    let file = File::open(file_path).expect("file should open read only");
    let json_data: Value = from_reader(file).expect("file should be proper JSON");

    for sec in 0..num_of_secs {
        // println!("sec {:?}", sec,);
        // user, workload for that user
        let mut sec_wd = HashMap::<usize, Vec<(i32, String)>>::with_capacity(100);

        let urls_now = match json_data.get(sec.to_string()) {
            Some(val) => val.as_array(),
            None => continue,
        };
        let all_seq = match urls_now {
            Some(v) => v,
            None => continue,
        };

        for seq in all_seq {
            let visits = seq.as_array().unwrap();
            // println!("\n sec {:?}, break: {:?}", sec, visits);

            let mut vec_wd = Vec::new();
            for idx in 0..visits.len() {
                let time_url = visits[idx].as_str().unwrap().to_string();
                let v: Vec<&str> = time_url.split(':').collect();
                let wait_time: i32 = v[1].parse().unwrap();
                vec_wd.push((wait_time, v[0].to_string()));
            }
            let magic = get_magic_num(num_of_user);
            if sec == 599 {
                println!("\n{:?} {:?}", magic, vec_wd);
            }
            sec_wd.insert(magic, vec_wd);
        }
        workload.insert(sec, sec_wd);
    }
    Ok(workload)
}

fn main() {
    let workload = rdr_load_workload(
        "/home/jethros/dev/silver-octo-spoon/workload_tempaltes/rdr_workload.json".to_string(),
        600,
        100,
    )
    .unwrap();
    // println!("{:?}", workload);
}
