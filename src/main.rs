mod data;
mod api;
mod processing;
mod display;

use std::fs::OpenOptions;
use clap::Parser;
use crate::api::fetch_crates;
use crate::processing::process_crates;
use crate::display::{display_crates, get_user_selection};
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "Rust Crate Recommender")]
#[command(about = "Finds relevant Rust crates based on keywords")]
struct Cli {
    keyword: Option<String>, // Now optional since we support interactive mode
    #[arg(default_value_t = 5)]
    max_results: usize,
}

/// Saves selected dependencies to a file.
fn save_dependencies(dependencies: &[(String, String)]) {
    if dependencies.is_empty() {
        println!("⚠️ No dependencies to save.");
        return;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("dependencies.txt")
        .expect("Failed to open file");

    for (crate_name, doc_link) in dependencies {
        writeln!(file, "{} - Documentation: {}", crate_name, doc_link)
            .expect("Failed to write to file");
    }

    println!("📂 Dependencies saved to `dependencies.txt`");
}

#[tokio::main]
async fn main() {
    let mut dependencies: Vec<(String, String)> = Vec::new();

    loop {
        print!("Enter a keyword to search for crates (or 'exit' to quit): ");
        io::stdout().flush().unwrap();

        let mut keyword = String::new();
        io::stdin().read_line(&mut keyword).unwrap();
        let keyword = keyword.trim();

        if keyword.eq_ignore_ascii_case("exit") {
            println!("Exiting the program...");
            save_dependencies(&dependencies);
            break;
        }

        match fetch_crates(keyword).await {
            Ok(crates) => {
                if crates.is_empty() {
                    println!("No results found for '{}'. Try another keyword.", keyword);
                    continue;
                }

                // Process the results before displaying
                let processed_crates = process_crates(crates);
                display_crates(&processed_crates);

                if let Some((crate_name, doc_link)) = get_user_selection(&processed_crates) {
                    dependencies.push((crate_name.clone(), doc_link.clone()));
                    println!("Added '{}' to your dependencies.", crate_name);
                }
            },
            Err(e) => println!("Error fetching crates: {}", e),
        }
    }
}
