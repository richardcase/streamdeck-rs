#[macro_use]
extern crate log;
extern crate simplelog;
use simplelog::{ColorChoice, LevelFilter, TermLogger, TerminalMode};

use streamdeck::{Filter, StreamDeck};

extern crate structopt;
use structopt::StructOpt;

extern crate hidapi;

#[derive(StructOpt)]
#[structopt(name = "streamdeck-rs", about = "a streamdeck cli")]
struct Options {
    #[structopt(flatten)]
    filter: Filter,

    #[structopt(long = "log-level", default_value = "info")]
    /// Enable verbose logging
    level: LevelFilter,
}

fn main() {
    let opts = Options::from_args();

    let mut config = simplelog::ConfigBuilder::new();
    config.set_time_level(LevelFilter::Off);

    TermLogger::init(
        opts.level,
        config.build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let hid_api = hidapi::HidApi::new().unwrap();
    for device in hid_api.device_list() {
        println!("{:#?}", device)
    }

    let dev = hid_api.device_list().next().unwrap();

    // let mut deck = match StreamDeck::connect(opts.filter.vid, opts.filter.pid, opts.filter.serial) {
    let mut deck = match StreamDeck::connect_with_hid(
        &hid_api,
        dev.vendor_id(),
        dev.product_id(),
        opts.filter.serial,
    ) {
        Ok(d) => d,
        Err(e) => {
            error!("error connecting to streamdeck: {:?}", e);
            return;
        }
    };

    let serial = deck.serial().unwrap();
    info!(
        "Connected to device (vid: {:04x} pid: {:04x} serial: {})",
        opts.filter.vid, opts.filter.pid, serial
    );

    deck.set_button_text()
}
