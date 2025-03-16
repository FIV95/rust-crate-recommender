use crate::data::CrateInfo;

/// Sorts crates by downloads and filters out entries with no descriptions.
pub fn process_crates(mut crates: Vec<CrateInfo>) -> Vec<CrateInfo> {
    // Sort results by downloads (descending order)
    crates.sort_by(|a, b| b.downloads.cmp(&a.downloads));

    // Remove crates without descriptions
    crates.retain(|c| c.description.is_some());

    crates
}
