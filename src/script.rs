#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! reqwest = "0.12.15" # HTTP Client
//! serde = { version = "1.0.219", features = ["derive"] }  # Serialization/Deserialization
//! serde_json = "1.0.140"  # JSON Serialization/Deserialization
//! tokio = { version = "1", features = ["full"] } # Asynchronous Runtime
//! toml = "0.8.22" # TOML Parsing
//! ```

mod config;

use std::fs;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use config::Config;

/// This function retrieves all issues from a specified GitHub repository.
/// It uses the GitHub API to fetch issues and returns the response as a string (for now).
///
/// ## Arguments
/// - `config`: A reference to a `Config` struct that contains the GitHub API token, owner, and repository name.
///
/// See: https://docs.github.com/en/rest/issues/issues?apiVersion=2022-11-28#list-repository-issues
async fn get_all_issues(config: &Config) -> String {
    // Setup the Reqwest client.
    let client = reqwest::Client::new();
    // Construct the URL for the GitHub API request.
    let url = format!(
        "https://api.github.com/repos/{}/{}/issues",
        config.github.owner, config.github.repository
    );

    // Send the GET request to the GitHub API.
    let response = client.get(url)
        .header(USER_AGENT, "blog-friend-links-data-generator by iXOR Technology")
        .header(ACCEPT, "application/vnd.github+json")
        .header(AUTHORIZATION, format!("Bearer {}", config.github.token))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await;

    response.unwrap().text().await.unwrap()
}

#[tokio::main]
async fn main() {
    // Read the config.toml file and parse it.
    let config_file: String = fs::read_to_string("config.toml").expect("Failed to Read Configuration File");
    let config: Config = toml::from_str(&config_file).expect("Failed to Parse Configuration");

    println!("Github Token: {}", config.github.token);
    println!("Github Owner: {}", config.github.owner);
    println!("Github Repository: {}", config.github.repository);

    println!("Generation Label: {}", config.generation.label);
    println!("Generation Groups: {:?}", config.generation.groups);
    println!("Sort by Updated Time: {}", config.generation.sort_by_updated_time);

    // Call the function to get all issues from the GitHub repository.
    let issues = get_all_issues(&config).await;
}
