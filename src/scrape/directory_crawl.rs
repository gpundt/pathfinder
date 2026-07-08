use super::connectivity_check;
use log::{debug, error, info, trace, warn};
use std::fs::File;
use std::io::{BufRead, BufReader};

const MAX_CONCURRENT_REQUESTS: usize = 5;

/// Given a filepath, parse each line and populate wordlist Vec<String>
pub fn parse_word_list(filepath: &String) -> Result<Vec<String>, String> {
    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(e) => return Err(e.to_string()),
    };
    let reader = BufReader::new(file);
    let mut word_list: Vec<String> = Vec::new();

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line_string) => line_string,
            Err(e) => return Err(e.to_string()),
        };

        if !line.trim().is_empty() {
            word_list.push(line);
        }
    }

    info!("{} Words Parsed from '{}'", word_list.len(), filepath);
    Ok(word_list)
}

/// Using wordlist, contacts each URL to see if it exists
pub fn directory_crawl(url: &String, wordlist: Vec<String>) -> Result<(), String> {
    info!("Beginning Directory Crawl on '{}'", url);

    for p in wordlist {
        let page_path: String = p;

        // Strip '/' from beginning of path
        let formatted_path = page_path.strip_prefix("/").unwrap_or(&page_path);

        let current_url: String = format!("{}/{}", url, formatted_path);

        connectivity_check::query(&current_url);
    }

    Ok(())
}
