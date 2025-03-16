use crate::data::CrateInfo;
use colored::*;
use std::io::{self, Write};

/// Displays formatted crate recommendations, including documentation links.
pub fn display_crates(crates: &[CrateInfo]) {
    println!("\n{}", "Recommended Rust Crates:".bold().green());

    for (index, crate_info) in crates.iter().enumerate().take(10) {
        let doc_link = crate_info.documentation.as_deref().unwrap_or("No documentation available");

        println!(
            "{}. {} - {} ({} downloads)\n   📖 Docs: {}",
            index + 1,
            crate_info.id.cyan(),
            crate_info.description.as_deref().unwrap_or("No description").yellow(),
            crate_info.downloads.to_string().red(),
            doc_link.blue()
        );
    }
}

/// Handles user input to select dependencies.
pub fn get_user_selection(crates: &[CrateInfo]) -> Option<(String, String)> {
    print!("Enter the number of a crate to add to your dependencies (or press Enter to skip): ");
    io::stdout().flush().unwrap();

    let mut selection = String::new();
    io::stdin().read_line(&mut selection).unwrap();

    if let Ok(index) = selection.trim().parse::<usize>() {
        if index > 0 && index <= crates.len() {
            let selected_crate = &crates[index - 1];
            let crate_name = selected_crate.id.clone();
            let doc_link = selected_crate.documentation.clone().unwrap_or_else(|| "No documentation available".to_string());
            return Some((crate_name, doc_link));
        }
    }
    None
}
