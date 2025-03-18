# Rust Crate Recommender

## 📌 Overview
Rust Crate Recommender is a command-line tool that helps Rust developers discover and manage useful crates from [crates.io](https://crates.io). By searching for keywords, it retrieves popular crates, sorts them by downloads, and provides an interactive way to add dependencies to a file for easy reference.

## 🚀 Features
- 🔎 Search for crates by keyword
- 📊 Sort results by download count
- 📝 Display crate descriptions and documentation links
- 🏗️ Filter out crates with missing descriptions
- 💾 Save selected dependencies for future use

## 🛠️ Installation
Ensure you have [Rust](https://www.rust-lang.org/) and Cargo installed. Then, clone the repository and build the project:

```sh
git clone https://github.com/FIV95/rust-crate-recommender.git
cd rust-crate-recommender
cargo build --release
