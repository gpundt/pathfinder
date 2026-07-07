use url::{ParseError, Url};

/// Helper function to get root URL of user-provided URL
pub fn get_root_url(url: &String) -> Result<String, ParseError> {
    let parsed = Url::parse(url)?;

    if let Some(host) = parsed.host_str() {
        Ok(format!("{}://{}", parsed.scheme(), host))
    } else {
        Ok(format!("{}:{}", parsed.scheme(), parsed.path()))
    }
}
