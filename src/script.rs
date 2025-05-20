#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! serde = { version = "1.0.219", features = ["derive"] }  # Serialization/Deserialization
//! toml = "0.8.22" # TOML Parsing
//! ```

use serde::Deserialize;

/// The structure of the script configuration.
///
/// It contains two parts:
/// - `github`: Configuration for GitHub API access.
/// - `generation`: Configuration for the data generation process.
#[derive(Deserialize)]
struct Config {
    github: GithubConfig,
    generation: GenerationConfig
}

/// The structure of the GitHub configuration.
///
/// It contains:
/// - `token`: The GitHub API access token.
/// - `owner`: The owner of the GitHub repository where issues to be processed are located.
/// - `repository`: The name of the GitHub repository where issues to be processed are located.
#[derive(Deserialize)]
struct GithubConfig {
    token: String,
    owner: String,
    repository: String,
}

/// The structure of the data generation configuration.
///
/// It contains:
/// - `label`: The label added to the issues to be included in the generated data.
/// - `groups`: The groups that separate issues and generated data into different categories.
/// - `sort_by_updated_time`: Whether to sort the issues by their updated time or creation time.
#[derive(Deserialize)]
struct GenerationConfig {
    label: String,
    groups: Vec<String>,
    sort_by_updated_time: bool,
}

fn main() {
    // Read the config.toml file and parse it.
    let config_file: String = std::fs::read_to_string("config.toml").expect("Failed to Read Configuration File");
    let config: Config = toml::from_str(&config_file).expect("Failed to Parse Configuration");

    println!("Github Token: {}", config.github.token);
    println!("Github Owner: {}", config.github.owner);
    println!("Github Repository: {}", config.github.repository);

    println!("Generation Label: {}", config.generation.label);
    println!("Generation Groups: {:?}", config.generation.groups);
    println!("Sort by Updated Time: {}", config.generation.sort_by_updated_time);
}
