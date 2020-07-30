extern crate base64;
extern crate tiny_http;

use failure::Fallible;
use std::fs;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

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

pub fn tab_create_unwrap(hostname: String) -> Fallible<()> {
    // create_unwrap a headless browser, navigate to wikipedia.org, wait for the page
    // to render completely, take a screenshot of the entire page
    // in JPEG-format using 75% quality.
    println!("RDR entry point",);
    let options = LaunchOptionsBuilder::default()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");

    let browser = Browser::new(options)?;
    println!("RDR browser",);
    let tab = browser.wait_for_initial_tab()?;
    println!("RDR tab",);

    println!("RDR entry point",);
    let options2 = LaunchOptionsBuilder::default()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");

    let browser2 = Browser::new(options2)?;
    println!("RDR browser",);
    let tab2 = browser2.wait_for_initial_tab()?;
    println!("RDR tab",);

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
            // println!("\nDEBUG: url content: {:?}", intercepted.request.url);
            // println!("\nDEBUG: {:?}", intercepted.request);
            if intercepted.request.url.ends_with(".js") {
                // println!("DEBUG: jackpot! We have JS code",);
                let js_body = r#"document.body.appendChild(document.create_unwrapElement("hr"));"#;
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

                let base64_response = base64::encode(&wrapped_writer);

                RequestInterceptionDecision::Response(base64_response)
            } else {
                RequestInterceptionDecision::Continue
            }
        }),
    )?;

    println!("RDR tab enable request",);

    let responses = Arc::new(Mutex::new(Vec::new()));

    tab.enable_response_handling(Box::new(move |response, fetch_body| {
        // NOTE: you can only fetch the body after it's been downloaded, which might be some time
        // after the initial 'response' (with status code, headers, etc.) has come back. hence this
        // sleep:
        // println!("\nDEBUG: Response {:?}", response);
        sleep(Duration::from_millis(100));
        let body = fetch_body().unwrap();
        // println!("\nDEBUG: Response body: {:?}", body);
        responses.lock().unwrap().push((response, body));
    }))?;

    println!("RDR tab enable response",);

    println!("\nhostname is: {:?}\n", hostname);
    // let jpeg_data = tab.navigate_to(&hostname)?.wait_until_navigated()?;

    let http_hostname = "http://".to_string() + &hostname;
    let jpeg_data = tab.navigate_to(&http_hostname)?.wait_until_navigated()?;

    Ok(())
}

#[allow(dead_code)]
pub fn tab_create_unwrap_unwrap(hostname: String) {
    // create_unwrap a headless browser, navigate to wikipedia.org, wait for the page
    // to render completely, take a screenshot of the entire page
    // in JPEG-format using 75% quality.
    println!("RDR entry point",);
    let options = LaunchOptionsBuilder::default()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    println!("RDR options",);
    let browser = Browser::new(options).unwrap();
    println!("RDR browser",);
    let tab = browser.wait_for_initial_tab().unwrap();
    println!("RDR tab",);

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
            println!("\nDEBUG: url content: {:?}", intercepted.request.url);
            println!("\nDEBUG: {:?}", intercepted.request);
            if intercepted.request.url.ends_with(".js") {
                println!("DEBUG: jackpot! We have JS code",);
                let js_body = r#"document.body.appendChild(document.create_unwrapElement("hr"));"#;
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

                let base64_response = base64::encode(&wrapped_writer);

                RequestInterceptionDecision::Response(base64_response)
            } else {
                RequestInterceptionDecision::Continue
            }
        }),
    )
    .unwrap();
    println!("RDR tab enable request",);

    let responses = Arc::new(Mutex::new(Vec::new()));

    tab.enable_response_handling(Box::new(move |response, fetch_body| {
        // NOTE: you can only fetch the body after it's been downloaded, which might be some time
        // after the initial 'response' (with status code, headers, etc.) has come back. hence this
        // sleep:
        println!("\nDEBUG: Response {:?}", response);
        sleep(Duration::from_millis(100));
        let body = fetch_body().unwrap();
        println!("\nDEBUG: Response body: {:?}", body);
        responses.lock().unwrap().push((response, body));
    }))
    .unwrap();

    println!("RDR tab enable response",);

    // hostname is String,
    println!("\nHostname: {:?}\n", hostname);
    let http_hostname = "http://".to_string() + &hostname;
    let jpeg_data = tab
        .navigate_to(&http_hostname)
        .unwrap()
        .wait_until_navigated()
        .unwrap();
}

// pub fn extract_http_request(payload: &[u8]) -> Result<String, HttpRequestNotExtractedError> {
//     // if the first three bytes are "GET" or "POS", there's a chance the packet is HTTP
//     // if the first three bytes are 0x16, 0x30, 0x00-0x03, there's a chance the packet is TLS
//
//     let get: &[u8] = &[71, 69, 84]; // GET
//     let post: &[u8] = &[80, 79, 83]; // POS
//     let http: &[u8] = &[72, 84, 84]; // HTT
//     let tls0: &[u8] = &[22, 3, 0];
//     let tls1: &[u8] = &[22, 3, 1];
//     let tls2: &[u8] = &[22, 3, 2];
//     let tls3: &[u8] = &[22, 3, 3];
//
//     let (head, _) = payload.split_at(3);
//
//     if head == get {
//         let payload_str = match std::str::from_utf8(payload) {
//             Ok(s) => s.to_string(),
//             Err(_) => return Err(HttpRequestNotExtractedError),
//         };
//
//         let get_request = HttpRequest::new(&payload_str).unwrap();
//         let headers = get_request.headers;
//
//         let mut _iterator = headers.iter();
//
//         while let Some(h) = _iterator.next() {
//             if h.name == HttpHeaderName::Host {
//                 println!("\nImportant: issuing a HTTP request for {:?}", h.value);
//                 return Ok(h.value.clone());
//             } else {
//                 continue;
//             }
//         }
//         return Err(HttpRequestNotExtractedError);
//     } else {
//         Err(HttpRequestNotExtractedError)
//     }
// }

fn main() {
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());

    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());

    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());

    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());

    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());

    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());

    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());

    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());

    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
    tab_create_unwrap("lobste.rs".to_string());
}
