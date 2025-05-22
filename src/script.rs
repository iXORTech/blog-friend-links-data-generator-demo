#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! chrono = "0.4.41"   # Date and Time Library
//! reqwest = "0.12.15" # HTTP Client
//! serde = { version = "1.0.219", features = ["derive"] }  # Serialization/Deserialization
//! serde_json = "1.0.140"  # JSON Serialization/Deserialization
//! tokio = { version = "1", features = ["full"] } # Asynchronous Runtime
//! toml = "0.8.22" # TOML Parsing
//! ```

mod config;
mod github_api_responses;

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
async fn get_all_issues(config: &Config) -> Vec<github_api_responses::Issue> {
    // Setup the Reqwest client.
    let client = reqwest::Client::new();
    // Construct the URL for the GitHub API request.
    let url = format!(
        "https://api.github.com/repos/{}/{}/issues",
        config.github.owner, config.github.repository
    );

    // Send the GET request to the GitHub API.
    let res = client.get(url)
        .header(USER_AGENT, "blog-friend-links-data-generator by iXOR Technology")
        .header(ACCEPT, "application/vnd.github+json")
        .header(AUTHORIZATION, format!("Bearer {}", config.github.token))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await;

    // Check if the request was successful.
    match res {
        Ok(res) => {
            if res.status().is_success() {
                let res_body = res.text().await;
                match res_body {
                    Ok(body) => {
                        // Deserialize the response body into a vector of `Issue` structs and return it.
                        serde_json::from_str(&body).expect("Failed to Parse Response")
                    }
                    Err(e) => {
                        panic!("Failed to Read Response: {}", e);
                    }
                }
            } else {
                panic!("Failed to Fetch Issues: {}", res.status());
            }
        }
        Err(e) => {
            panic!("Error Sending Request: {}", e);
        }
    }
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
    println!("Sort by Updated Time: {}", config.generation.sort_by_updated_time);

    println!("Groups:");
    for group in &config.groups {
        println!("  - Name: {}", group.name);
        println!("    Description: {}", group.description);
        println!("    Label: {}", group.label);
    }

    // Call the function to get all issues from the GitHub repository.
    let issues = get_all_issues(&config).await;

    // Print the issues to the console.
    println!();
    for issue in issues {
        println!("Issue ID: {}", issue.id);
        println!("Issue Title: {}", issue.title);
        println!("Issue State: {}", issue.state);
        println!("Issue URL: {}", issue.url);
        println!("Issue Created At: {}", issue.created_at());
        println!("Issue Updated At: {}", issue.updated_at());
        println!("Issue Closed At: {:?}", issue.closed_at());
        println!("Labels: {:?}", issue.labels);
        println!();
    }
}
