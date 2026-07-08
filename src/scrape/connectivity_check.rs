use futures::{StreamExt, stream};
use log::{info, warn};
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

/// Helper function to query indvidual
pub fn query(url: &str) -> () {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    match client.get(url).send() {
        Ok(response) => {
            let status = response.status();
            if status != StatusCode::OK {
                warn!("{:<40} {}", url, status);
            } else {
                info!("{:<40} Found: {}", url, status);
            }
        }
        Err(err) => warn!("{:<40} {}", url, err),
    }

    ()
}
