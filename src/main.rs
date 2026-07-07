mod core;
mod scrape;

use clap::Parser;
use core::{arguments::Args, logging};
use log::{debug, error, info, trace, warn};
use scrape::connectivity_check;

fn main() {
    let args: Args = Args::parse();

    logging::_set_log_level(args.verbose);

    info!("{:<10}{}", "URL:", args.url);
    info!("{:<10}{} sec", "Timeout:", args.timeout);
    debug!("{:<10}{}", "Verbose:", args.verbose);

    if !connectivity_check::validate(&args.url) {
        error!("Failed to connect to {}", args.url);
    }
}
