use reqwest::{StatusCode, blocking::Client};
use std::time::Duration;

pub fn validate(url: &str) -> Result<(), String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    match client.get(url).send() {
        Ok(response) => match response.status() {
            StatusCode::OK => Ok(()),
            _ => Err(format!("Status Code: {}", response.status())),
        },
        Err(err) => {
            if err.is_timeout() {
                return Err(format!("Failed to connect: timeout reached"));
            } else if err.is_connect() {
                return Err(format!("Failed to connect to {}", url));
            } else {
                return Err(format!("{}", err));
            }
        }
    }
}
