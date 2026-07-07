use reqwest::blocking::Client;
use std::time::Duration;

pub fn validate(url: &str) -> bool {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    match client.get(url).send() {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}
