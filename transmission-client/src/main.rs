extern crate transmission;
use transmission::{Client, ClientConfig};

fn main() {
    let config_dir = "config/";
    let download_dir = "downloads/";

    // big-buck-bunny.torrent  cosmos-laundromat.torrent  sintel.torrent  tears-of-steel.torrent  wired-cd.torrent
    let free_path = "free-torrents/big-buck-bunny.torrent";
    // debian-mac-10.1.0-amd64-netinst.iso.torrent  ubuntu-16.04.6-desktop-amd64.iso.torrent  ubuntu-16.04.6-server-i386.iso.torrent        ubuntu-19.04-desktop-amd64.iso.torrent
    // FreeBSD-12.1-BETA3.torrent                   ubuntu-16.04.6-desktop-i386.iso.torrent   ubuntu-18.04.3-desktop-amd64.iso.torrent      ubuntu-19.04-live-server-amd64.iso.torrent
    // OpenBSD-6.6-amd64.iso.torrent                ubuntu-16.04.6-server-amd64.iso.torrent   ubuntu-18.04.3-live-server-amd64.iso.torrent
    let linux_path = "linux-torrents/ubuntu-19.04-desktop-amd64.iso.torrent";

    let c = ClientConfig::new()
        .app_name("testing")
        .config_dir(config_dir)
        .download_dir(download_dir);

    let mut c = Client::new(c);

    let t1 = c.add_torrent_file(free_path).unwrap();
    let t2 = c.add_torrent_file(linux_path).unwrap();
    t1.start();
    t2.start();

    // Run until done
    while t1.stats().percent_complete < 1.0 {
        print!("{:#?}\r", t1.stats().percent_complete);
    }
    while t2.stats().percent_complete < 1.0 {
        print!("{:#?}\r", t2.stats().percent_complete);
    }
    c.close();
}
