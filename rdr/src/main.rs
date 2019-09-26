// take screenshot and query wiki

use std::{thread, time};

use std::fs;

use failure::Fallible;

use headless_chrome::{protocol::page::ScreenshotFormat, Browser, LaunchOptionsBuilder};

fn main() -> Fallible<()> {
    // Create a headless browser, navigate to wikipedia.org, wait for the page
    // to render completely, take a screenshot of the entire page
    // in JPEG-format using 75% quality.
    let options = LaunchOptionsBuilder::default()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser = Browser::new(options)?;

    let tab = browser.wait_for_initial_tab()?;
    let jpeg_data = tab
        .navigate_to("https://www.google.com/search?q=juste+debout&sxsrf=ACYBGNQavKhe1ClYX728iA_voB8guZUcJw:1569524992306&source=lnms&tbm=isch&sa=X&ved=0ahUKEwjkoYedmO_kAhXwm-AKHUyhBlEQ_AUIEygD&biw=1684&bih=841")?
        .wait_until_navigated()?
        .capture_screenshot(ScreenshotFormat::JPEG(Some(75)), None, true)?;
    fs::write("screenshot.jpg", &jpeg_data)?;

    let jpeg_data = tab
        .navigate_to("https://www.youtube.com/watch?v=1uflg7LDmzI")?
        .wait_until_navigated()?
        .capture_screenshot(ScreenshotFormat::JPEG(Some(75)), None, true)?;
    fs::write("screenshot_youtube_1.jpg", &jpeg_data)?;

    // Browse to the WebKit-Page and take a screenshot of the infobox.
    let png_data = tab
        .navigate_to("https://en.wikipedia.org/wiki/WebKit")?
        .wait_for_element("#mw-content-text > div > table.infobox.vevent")?
        .capture_screenshot(ScreenshotFormat::PNG)?;
    fs::write("screenshot.png", &png_data)?;

    let ten_millis = time::Duration::from_millis(10);
    let now = time::Instant::now();

    thread::sleep(time::Duration::from_secs(100));

    println!("Screenshots successfully created.");
    Ok(())
}
