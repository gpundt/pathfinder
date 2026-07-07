use log::{debug, error, info, trace, warn};
use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn parse_word_list(filepath: &String) -> Result<Vec<String>, String> {
    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(e) => return Err(e.to_string())
    };
    let reader = BufReader::new(file);
    let mut word_list: Vec<String> = Vec::new();

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line_string) => line_string,
            Err(e) => return Err(e.to_string())
        };

        if !line.trim().is_empty(){
            word_list.push(line);
        }            
    }

    info!("{} Words Parsed from '{}'", word_list.len(), filepath);
    Ok(word_list)
}

///
pub fn directory_crawl(url: &String, wordlist: Vec<String>) -> Result<(), String> {
    info!("Beginning Directory Crawl on '{}'", url);
    

    Ok(())
}
