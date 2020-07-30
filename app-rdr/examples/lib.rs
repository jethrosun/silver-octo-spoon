use failure::Fallible;
use headless_chrome::LaunchOptions;
use headless_chrome::{browser::context::Context, Browser, Tab};
use serde_json::{from_reader, Result, Value};
use std::collections::HashMap;
use std::fs::File;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::vec::Vec;

/// Construct the workload from the session file.
///
/// https://kbknapp.github.io/doapi-rs/docs/serde/json/index.html

/// Construct the workload from the session file.
///
/// https://kbknapp.github.io/doapi-rs/docs/serde/json/index.html
pub fn rdr_load_workload(
    file_path: String,
    num_of_secs: usize,
    num_of_user: usize,
) -> Result<HashMap<usize, Vec<(u64, String, usize)>>> {
    // time in second, workload in that second
    // let mut workload = HashMap::<usize, HashMap<usize, Vec<(u64, String)>>>::with_capacity(num_of_secs);
    let mut workload = HashMap::<usize, Vec<(u64, String, usize)>>::with_capacity(num_of_secs);

    let file = File::open(file_path).expect("file should open read only");
    let json_data: Value = from_reader(file).expect("file should be proper JSON");
    // println!("{:?}", json_data);

    for sec in 0..num_of_secs {
        // println!("sec {:?}", sec,);
        // user, workload for that user
        //
        // sec = 266
        // sec_wd = {'86': [39, 'thumb.brandreachsys.com'], '23': [42, 'facebook.com'], '84': [86, 'ynetnews.com'], '33': [284, 'techbargains.com'], '9': [309, 'bing.com'], '76': [357, 'eventful.com'], '43': [365, 'ad.yieldmanager.com'], '63': [468, 'ads.brazzers.com'], '72': [520, 'sidereel.com'], '57': [586, 'daum.net'], '81': [701, 'target.com'], '95': [732, 'lezhin.com'], '88': [802, 'nba.com'], '49': [827, 'web4.c2.cyworld.com'], '27': [888, 'hv3.webstat.com'], '98': [917, 'youtube.com']}
        // let mut sec_wd = HashMap::<usize, Vec<(u64, String)>>::with_capacity(100);
        // we keep track of the millisecond appeared
        let mut millis: Vec<(u64, String, usize)> = Vec::new();

        // println!("{:?} {:?}", sec, json_data.get(sec.to_string()));
        let urls_now = match json_data.get(sec.to_string()) {
            Some(val) => val,
            None => continue,
        };
        // println!("{:?}", urls_now);
        for user in 0..num_of_user {
            let urls = match urls_now.get(user.to_string()) {
                Some(val) => val.as_array(),
                None => continue,
            };
            // println!("{:?}", urls.unwrap());

            if urls.unwrap()[1].as_str().unwrap().to_string() == "32.wcmcs.net" {
                continue;
            } else if urls.unwrap()[1].as_str().unwrap().to_string()
                == "provider-directory.anthem.com"
            {
                continue;
            } else if urls.unwrap()[1].as_str().unwrap().to_string() == "kr.sports.yahoo.com" {
                continue;
            } else if urls.unwrap()[1].as_str().unwrap().to_string() == "desktopfw.weather.com" {
                continue;
            } else if urls.unwrap()[1].as_str().unwrap().to_string() == "arienh4.net.nyud.net" {
                continue;
            } else if urls.unwrap()[1].as_str().unwrap().to_string() == "hv3.webstat.com" {
                continue;
            } else if urls.unwrap()[1].as_str().unwrap().to_string() == "rs.mail.ru" {
                continue;
            } else if urls.unwrap()[1].as_str().unwrap().to_string() == "arienh4.net.nyud.net" {
                continue;
            } else if urls.unwrap()[1].as_str().unwrap().to_string() == "apps.facebook.com" {
                continue;
            } else if urls.unwrap()[1].as_str().unwrap().to_string() == "ads.adultadvertising.net" {
                continue;
            } else if urls.unwrap()[1].as_str().unwrap().to_string() == "reuters.com" {
                continue;
            } else if urls.unwrap()[1].as_str().unwrap().to_string() == "pn1.adserver.yahoo.com" {
                continue;
            } else if urls.unwrap()[1].as_str().unwrap().to_string() == "bbc.co.uk" {
                continue;
            } else if urls.unwrap()[1].as_str().unwrap().to_string() == "ad.yieldmanager.com" {
                continue;
            } else if urls.unwrap()[1].as_str().unwrap().to_string() == "wikipedia.org" {
                continue;
            } else if urls.unwrap()[1].as_str().unwrap().to_string() == "collegehumor.com" {
                continue;
            } else if urls.unwrap()[1].as_str().unwrap().to_string() == "bbc.co.uk" {
                continue;
            } else if urls.unwrap()[1].as_str().unwrap().to_string() == "bbc.co.uk" {
                continue;
            } else if urls.unwrap()[1].as_str().unwrap().to_string() == "bbc.co.uk" {
                continue;
            } else {
                millis.push((
                    urls.unwrap()[0].as_u64().unwrap(),
                    urls.unwrap()[1].as_str().unwrap().to_string(),
                    user,
                ));
            }

            // if urls.unwrap()[1].as_str().unwrap().to_string() == "32.wcmcs.net" {
            //     println!("{:?}", urls.unwrap()[1].as_str().unwrap().to_string());
            //     millis.push((
            //         urls.unwrap()[0].as_u64().unwrap(),
            //         urls.unwrap()[1].as_str().unwrap().to_string(),
            //         user,
            //     ));
            // } else {
            //     continue;
            // }
        }
        millis.sort();

        // sec_wd.insert(millis);

        // {'96': [53, 'video.od.visiblemeasures.com'],
        //  '52': [104, 'drift.qzone.qq.com'],
        //  '15': [153, 'club.myce.com'],
        //  '78': [180, 'ad.admediaprovider.com'],
        //  '84': [189, 'inkido.indiana.edu'],
        //  '34': [268, 'waterjet.net'],
        //  '97': [286, 'apple.com'],
        //  '6': [317, 'southparkstudios.com'],
        //  '14': [362, 'en.wikipedia.org'],
        //  '27': [499, 'google.com'],
        //  '42': [619, 'womenofyoutube.mevio.com'],
        //  '75': [646, 'news.msn.co.kr'],
        //  '30': [750, 'hcr.com'],
        //  '61': [759, 'blogs.medicine.iu.edu'],
        //  '70': [815, 'mini.pipi.cn'],
        //  '54': [897, 'msn.foxsports.com'],
        //  '29': [926, 'target.com']}
        // if sec == 599 {
        //     println!("{:?}, ", millis);
        // }
        // println!("\n{:?} {:?}", sec, millis);
        workload.insert(sec, millis);
    }
    Ok(workload)
}

pub fn browser_create() -> Fallible<Browser> {
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .build()
            .expect("Could not find chrome-executable"),
    )?;
    // let tab = browser.wait_for_initial_tab()?;
    // tab.set_default_timeout(std::time::Duration::from_secs(100));

    // println!("Browser created",);
    Ok(browser)
}

pub fn user_browse(current_browser: &Browser, hostname: &String) -> Fallible<()> {
    // std::result::Result<(u128), (u128, failure::Error)> {
    let now = Instant::now();

    let tab = current_browser.wait_for_initial_tab()?;

    let https_hostname = "https://".to_string() + &hostname;

    // tab.navigate_to(&https_hostname)?.wait_until_navigated()?;
    tab.navigate_to(&https_hostname)?;

    let html = match tab.wait_for_element("html") {
        Ok(h) => {
            println!("ok");
            ()
        }
        Err(e) => {
            eprintln!("Query failed: {:?}", e);
            ()
        }
    };
    Ok(html)
}

// pub fn browser_ctx_create() -> Fallible<Context<'static>> {
//     let browser = browser_create().unwrap();
//     let ctx = browser.new_context().unwrap();
//
//     Ok(ctx)
// }

pub fn browser_tab_create() -> Fallible<Arc<Tab>> {
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .build()
            .expect("Could not find chrome-executable"),
    )?;
    let tab = browser.wait_for_initial_tab()?;

    // println!("Browser created",);
    Ok(tab)
}

pub fn user_tab_browse(
    current_tab: &Tab,
    hostname: &String,
) -> std::result::Result<(u128), (u128, failure::Error)> {
    let now = Instant::now();
    // println!("Entering user browsing",);
    // Doesn't use incognito mode
    //
    // let current_tab = match current_browser.new_tab() {
    //     Ok(tab) => tab,
    //     Err(e) => return Err((now.elapsed().as_micros(), e)),
    // };

    // Incogeneto mode
    //
    // let incognito_cxt = current_browser.new_context()?;
    // let current_tab: Arc<Tab> = incognito_cxt.new_tab()?;

    let https_hostname = "https://".to_string() + &hostname;

    // wait until navigated or not
    let navigate_to = match current_tab.navigate_to(&https_hostname) {
        Ok(tab) => tab,
        Err(e) => {
            return Err((now.elapsed().as_micros(), e));
        }
    };
    // let _ = current_tab.navigate_to(&https_hostname)?;
    let result = match navigate_to.wait_until_navigated() {
        Ok(_) => Ok(now.elapsed().as_micros()),
        Err(e) => Err((now.elapsed().as_micros(), e)),
    };

    result
}

pub fn simple_user_browse(
    current_browser: &Browser,
    hostname: &String,
) -> std::result::Result<(u128), (u128, failure::Error)> {
    let now = Instant::now();
    // println!("Entering user browsing",);
    // Doesn't use incognito mode
    //
    let current_tab = match current_browser.new_tab() {
        Ok(tab) => tab,
        Err(e) => return Err((now.elapsed().as_micros(), e)),
    };

    // Incogeneto mode
    //
    // let incognito_cxt = current_browser.new_context()?;
    // let current_tab: Arc<Tab> = incognito_cxt.new_tab()?;

    let https_hostname = "https://".to_string() + &hostname;

    // wait until navigated or not
    let result = match current_tab.navigate_to(&https_hostname) {
        Ok(_) => Ok(now.elapsed().as_micros()),
        // Err(e) => Err((now.elapsed().as_micros(), e)),
        Err(e) => Err((now.elapsed().as_micros(), e)),
    };

    // println!("{:?}", result);
    result
}

/// RDR proxy browsing scheduler.
///
///
// 4 [(4636, "fanfiction.net"), (9055, "bs.serving-sys.com")]
pub fn rdr_scheduler(
    now: Instant,
    pivot: &usize,
    num_of_ok: &mut usize,
    num_of_err: &mut usize,
    elapsed_time: &mut Vec<u128>,
    _num_of_users: &usize,
    current_work: Vec<(u64, String, usize)>,
    browser_list: &Vec<Browser>,
) {
    // println!("\npivot: {:?}", pivot);
    // println!("current work {:?}", current_work);

    for (milli, url, user) in current_work.into_iter() {
        println!("User {:?}: milli: {:?} url: {:?}", user, milli, url);
        println!("DEBUG: {:?} {:?}", now.elapsed().as_millis(), milli);

        if now.elapsed().as_millis() < milli as u128 {
            println!("DEBUG: waiting");
            let one_millis = Duration::from_millis(1);
            std::thread::sleep(one_millis);
        } else {
            println!("DEBUG: matched");
            match user_browse(&browser_list[user], &url) {
                Ok(elapsed) => {
                    println!("ok");
                    // *num_of_ok += 1;
                    // elapsed_time.push(elapsed);
                }
                // Err((elapsed, e)) => {
                Err(e) => {
                    println!("err");
                    // *num_of_err += 1;
                    // elapsed_time.push(elapsed);
                    println!("User {} caused an error: {:?}", user, e);
                    // println!("User {} caused an error", user,);
                }
            }
        }
    }

    // println!(
    //     "(pivot {}) RDR Scheduling: {:?} {:?}",
    //     pivot, num_of_ok, num_of_err
    // );
    // println!("(pivot {}) RDR Elapsed Time:  {:?}", pivot, elapsed_time);
}

pub fn rdr_scheduler_ng(
    now: Instant,
    pivot: &usize,
    num_of_ok: &mut usize,
    num_of_err: &mut usize,
    elapsed_time: &mut Vec<u128>,
    _num_of_users: &usize,
    current_work: Vec<(u64, String, usize)>,
    tab_list: &Vec<Arc<Tab>>,
) {
    // println!("\npivot: {:?}", pivot);
    // println!("current work {:?}", current_work);

    for (milli, url, user) in current_work.into_iter() {
        println!("User {:?}: milli: {:?} url: {:?}", user, milli, url);
        println!("DEBUG: {:?} {:?}", now.elapsed().as_millis(), milli);

        if now.elapsed().as_millis() < milli as u128 {
            println!("DEBUG: waiting");
            let one_millis = Duration::from_millis(1);
            std::thread::sleep(one_millis);
        } else {
            println!("DEBUG: matched");
            match user_tab_browse(&tab_list[user], &url) {
                Ok(elapsed) => {
                    println!("ok");
                    // *num_of_ok += 1;
                    // elapsed_time.push(elapsed);
                }
                Err((elapsed, e)) => {
                    println!("err");
                    // *num_of_err += 1;
                    // elapsed_time.push(elapsed);
                    println!("User {} caused an error: {:?}", user, e);
                    // println!("User {} caused an error", user,);
                }
            }
        }
    }
}
