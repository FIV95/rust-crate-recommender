use reqwest;
use reqwest::header::{HeaderMap, USER_AGENT};
use crate::data::{CrateResponse, CrateInfo};
use std::error::Error;

/// Fetches crates from the crates.io API based on a keyword search.
/// Returns a vector of `CrateInfo` structs or an error if the request fails.
pub async fn fetch_crates(keyword: &str) -> Result<Vec<CrateInfo>, Box<dyn Error>> {
    let url = format!("https://crates.io/api/v1/crates?q={}&sort=downloads", keyword);

    // Create a custom HTTP client with a User-Agent header
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "RustCrateRecommender/1.0 (contact@example.com)".parse()?);

    let client = reqwest::Client::new();
    let response = client.get(&url).headers(headers).send().await?.text().await?;

    // Deserialize
    let crate_response: CrateResponse = serde_json::from_str(&response)?;

    Ok(crate_response.crates) // Return raw results
}
