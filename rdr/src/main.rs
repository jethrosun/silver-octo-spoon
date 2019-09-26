extern crate base64;
extern crate tiny_http;

use failure::Fallible;
use std::fs;

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

fn main() -> Fallible<()> {
    // Create a headless browser, navigate to wikipedia.org, wait for the page
    // to render completely, take a screenshot of the entire page
    // in JPEG-format using 75% quality.
    let options = LaunchOptionsBuilder::default()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser = Browser::new(options)?;
    let tab = browser.wait_for_initial_tab()?;

    let patterns = vec![
        RequestPattern {
            url_pattern: None,
            resource_type: None,
            interception_stage: Some("HeadersReceived"),
        },
        RequestPattern {
            url_pattern: None,
            resource_type: None,
            interception_stage: Some("Request"),
        },
    ];

    tab.enable_request_interception(
        &patterns,
        Box::new(|transport, session_id, intercepted| {
            println!("DEBUG: url content: {:?}", intercepted.request.url);
            if intercepted.request.url.ends_with(".js") {
                println!("DEBUG: jackpot! We have JS code",);
                let js_body = r#"document.body.appendChild(document.createElement("hr"));"#;
                let js_response = tiny_http::Response::new(
                    200.into(),
                    vec![tiny_http::Header::from_bytes(
                        &b"Content-Type"[..],
                        &b"application/javascript"[..],
                    )
                    .unwrap()],
                    js_body.as_bytes(),
                    Some(js_body.len()),
                    None,
                );

                let mut wrapped_writer = Vec::new();
                js_response
                    .raw_print(&mut wrapped_writer, (1, 2).into(), &[], false, None)
                    .unwrap();
                println!("wrapper writer: {:?}", wrapped_writer);

                let base64_response = base64::encode(&wrapped_writer);

                println!("JS content: {:?}", base64_response);
                RequestInterceptionDecision::Response(base64_response)
            } else {
                RequestInterceptionDecision::Continue
            }
        }),
    )?;

    let jpeg_data = tab
        .navigate_to("https://tmz.com")?
        .wait_until_navigated()?
        .capture_screenshot(ScreenshotFormat::JPEG(Some(75)), None, true)?;
    fs::write("screenshot.jpg", &jpeg_data)?;

    // Browse to the WebKit-Page and take a screenshot of the infobox.
    let png_data = tab
        .navigate_to("https://en.wikipedia.org/wiki/WebKit")?
        .wait_for_element("#mw-content-text > div > table.infobox.vevent")?
        .capture_screenshot(ScreenshotFormat::PNG)?;
    fs::write("screenshot.png", &png_data)?;

    println!("Screenshots successfully created.");
    Ok(())
}
