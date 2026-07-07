mod core;
mod scrape;

use clap::Parser;
use core::{arguments::Args, logging};
use log::{debug, error, info, trace, warn};
use scrape::connectivity_check;
use std::process::exit;

fn main() {
    let args: Args = Args::parse();

    logging::_set_log_level(args.verbose);

    info!("{:<10}{}", "URL:", args.url);
    info!("{:<10}{} sec", "Timeout:", args.timeout);
    debug!("{:<10}{}", "Verbose:", args.verbose);

    match connectivity_check::validate(&args.url) {
        Ok(_) => info!("Successfully connected to {}", args.url),
        Err(err) => {
            error!("{}", err);
            exit(1);
        }
    }
}
