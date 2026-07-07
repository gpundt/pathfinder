mod core;
mod scrape;

use clap::Parser;
use core::{arguments::Args, logging, parse};
use log::{debug, error, info, trace, warn};
use scrape::{connectivity_check, directory_crawl};
use std::process::exit;

fn main() {
    let args: Args = Args::parse();

    logging::_set_log_level(args.verbose);

    let root_url: String = match parse::get_root_url(&args.url) {
        Ok(root_url) => root_url,
        Err(e) => {
            error!("Failed to parse URL: {}: {}", args.url, e);
            exit(1);
        }
    };

    info!("{:<10}{}", "URL:", root_url);
    info!("{:<10}{} sec", "Timeout:", args.timeout);
    debug!("{:<10}{}", "Verbose:", args.verbose);

    match connectivity_check::validate(&root_url) {
        Ok(_) => info!("Successfully connected to {}", root_url),
        Err(err) => {
            error!("{}", err);
            exit(1);
        }
    }

    let _ = directory_crawl::directory_crawl(&root_url);
}
