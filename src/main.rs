#[macro_use] extern crate log;
extern crate simplelog;
use simplelog::{TermLogger, LevelFilter, TerminalMode, ColorChoice};

use streamdeck::{StreamDeck, Filter, pids};

extern crate structopt;
use structopt::StructOpt;

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
    println!("Hello, world!");

    let opts = Options::from_args();

    let mut config = simplelog::ConfigBuilder::new();
    config.set_time_level(LevelFilter::Off);

    TermLogger::init(opts.level, config.build(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();

    // let mut deck = match StreamDeck::connect(opts.filter.vid, opts.filter.pid, opts.filter.serial) {
    let mut deck = match StreamDeck::connect(opts.filter.vid, pids::ORIGINAL_V2, opts.filter.serial) {
        Ok(d) => d,
        Err(e) => {
            error!("error connecting to streamdeck: {:?}", e);
            return
        }
    };

    let serial = deck.serial().unwrap();
    info!("Connected to device (vid: {:04x} pid: {:04x} serial: {})",
            opts.filter.vid, opts.filter.pid, serial);
}
