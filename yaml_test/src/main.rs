#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Application {
    application: Data,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Data {
    build: String,
    container_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<Vec<String>>,
}

fn main() {
    let filename = "example.yml";
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            let application_data: Application = serde_yaml::from_str(&content).unwrap();
            println!("{:?}", application_data.application.environment);
        }
        Err(error) => {
            println!("There is an error {}: {}", filename, error);
        }
    }
}
