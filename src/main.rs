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

    info!("{:<20}{}", "URL:", root_url);
    info!("{:<20}{} sec", "Timeout:", args.timeout);
    info!("{:<20}{}", "Word List:", args.wordlist);
    debug!("{:<20}{}", "Verbose:", args.verbose);

    match connectivity_check::validate(&root_url) {
        Ok(_) => info!("Successfully connected to '{}'", root_url),
        Err(err) => {
            error!("{}", err);
            exit(1);
        }
    }

    let word_list: Vec<String> = match directory_crawl::parse_word_list(&args.wordlist) {
        Ok(wordlist) => wordlist,
        Err(e) => {
            error!("Failed to generate wordlist: {}", e);
            exit(1);
        }
    };

    let _ = directory_crawl::directory_crawl(&root_url, word_list);
}
