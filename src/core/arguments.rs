use std::str;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Pathfinder")]
#[command(version = "1.0")]
#[command(about = "Web scraper - Written in Rust", long_about = None)]
pub struct Args {
    /// Target URL to spider and scrape
    #[arg(short, long)]
    pub url: String,

    /// Filepath of word list to use for page crawl
    #[arg(short, long)]
    #[arg(default_value_t = String::from("/etc/pathfinder/wordlists/ultimate-discovery.txt"))]
    pub wordlist: String,

    /// Turn on debug strings
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
