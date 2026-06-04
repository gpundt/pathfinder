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

    let log_level: LevelFilter = if args.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    colog::default_builder()
        .filter_level(log_level)
        .init();

    info!("{:<10}{}", "URL:", args.url);
    info!("{:<10}{}", "Timeout:", args.timeout);
    debug!("{:<10}{}", "Verbose:", args.verbose);
}
