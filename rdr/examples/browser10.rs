extern crate base64;
extern crate tiny_http;

use failure::Fallible;
use headless_chrome::browser::tab::RequestInterceptionDecision;
use headless_chrome::protocol::network::methods::RequestPattern;
use headless_chrome::protocol::network::Cookie;
use headless_chrome::protocol::runtime::methods::{RemoteObjectSubtype, RemoteObjectType};
use headless_chrome::protocol::RemoteError;
use headless_chrome::LaunchOptionsBuilder;
use headless_chrome::{
    protocol::browser::{Bounds, WindowState},
    protocol::page::ScreenshotFormat,
    Browser, Tab,
};
use std::fs;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Fallible<()> {
    let options = LaunchOptionsBuilder::default()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser = Browser::new(options)?;
    let tab = browser.wait_for_initial_tab()?;

    let options2 = LaunchOptionsBuilder::default()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser2 = Browser::new(options2)?;
    let tab2 = browser2.wait_for_initial_tab()?;

    let options3 = LaunchOptionsBuilder::default()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser3 = Browser::new(options3)?;
    let tab3 = browser3.wait_for_initial_tab()?;

    let options4 = LaunchOptionsBuilder::default()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser4 = Browser::new(options4)?;
    let tab4 = browser4.wait_for_initial_tab()?;

    let options5 = LaunchOptionsBuilder::default()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser5 = Browser::new(options5)?;
    let tab5 = browser5.wait_for_initial_tab()?;

    // 5

    let options6 = LaunchOptionsBuilder::default()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser6 = Browser::new(options6)?;
    let tab6 = browser6.wait_for_initial_tab()?;

    let options7 = LaunchOptionsBuilder::default()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser7 = Browser::new(options7)?;
    let tab7 = browser7.wait_for_initial_tab()?;

    let options8 = LaunchOptionsBuilder::default()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser8 = Browser::new(options8)?;
    let tab8 = browser8.wait_for_initial_tab()?;

    let options9 = LaunchOptionsBuilder::default()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser9 = Browser::new(options9)?;
    let tab9 = browser9.wait_for_initial_tab()?;

    let options10 = LaunchOptionsBuilder::default()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser10 = Browser::new(options10)?;
    let tab10 = browser10.wait_for_initial_tab()?;

    // 10

    let options11 = LaunchOptionsBuilder::default()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser11 = Browser::new(options11)?;
    let tab11 = browser11.wait_for_initial_tab()?;

    let options12 = LaunchOptionsBuilder::default()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser12 = Browser::new(options12)?;
    let tab12 = browser12.wait_for_initial_tab()?;

    let options13 = LaunchOptionsBuilder::default()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser13 = Browser::new(options13)?;
    let tab13 = browser13.wait_for_initial_tab()?;

    let options14 = LaunchOptionsBuilder::default()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser14 = Browser::new(options14)?;
    let tab14 = browser14.wait_for_initial_tab()?;

    let options15 = LaunchOptionsBuilder::default()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser15 = Browser::new(options15)?;
    let tab15 = browser15.wait_for_initial_tab()?;

    sleep(Duration::from_secs(300));

    Ok(())
}
