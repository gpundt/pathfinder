use std::str;

use clap::Parser;
use log::{LevelFilter, debug, error, info, trace, warn};


#[derive(Parser, Debug)]
#[command(name = "Pathfinder")]
#[command(version = "1.0")]
#[command(about = "Web scraper - Written in Rust", long_about = None)]
struct Args {
    /// Target URL to spider and scrape
    #[arg(short, long)]
    url: String,
    
    /// Timeout (in seconds)
    #[arg(short, long, default_value_t = 1)]
    timeout: u8,

    /// Turn on debug strings
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args: Args = Args::parse();

    _set_log_level(args.verbose);

    info!("{:<10}{}", "URL:", args.url);
    info!("{:<10}{} sec", "Timeout:", args.timeout);
    debug!("{:<10}{}", "Verbose:", args.verbose);
}

/// Helper function to set program's log level
/// 
/// verbose - User-provided verbose argument
fn _set_log_level(verbose: bool) {
    let log_level: LevelFilter = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    colog::default_builder()
        .filter_level(log_level)
        .init();

    ()
}
