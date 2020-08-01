extern crate base64;
extern crate tiny_http;

use failure::Fallible;
use headless_chrome::browser::tab::RequestInterceptionDecision;
use headless_chrome::protocol::network::methods::RequestPattern;
use headless_chrome::protocol::network::Cookie;
use headless_chrome::protocol::runtime::methods::{RemoteObjectSubtype, RemoteObjectType};
use headless_chrome::protocol::RemoteError;
use headless_chrome::{
    browser::context::Context,
    protocol::browser::{Bounds, WindowState},
    Browser, Tab,
};
use lib::*;
use std::fs;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::{Duration, Instant};

mod lib;

fn main() {
    let mut pivot = 1 as usize;

    let mut num_of_ok = 0;
    let mut num_of_err = 0;
    let mut elapsed_time: Vec<u128> = Vec::new();

    let browser = browser_create().unwrap();

    let now = Instant::now();

    loop {
        if now.elapsed().as_secs() == pivot as u64 {
            let min = pivot / 60;
            let rest_sec = pivot % 60;
            println!("\n{:?} min, {:?} second", min, rest_sec);
            let hostname = "google.com".to_string();

            user_browse(&browser, &hostname);

            pivot += 1;
        }
    }
}
