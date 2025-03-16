use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CrateInfo {
    pub id: String,
    pub description: Option<String>,
    pub downloads: u64,
    pub documentation: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct CrateResponse {
    pub crates: Vec<CrateInfo>,
}
