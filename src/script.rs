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

use config::Config;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use std::fs;
use std::collections::HashMap;
use crate::config::GroupConfig;

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

/// This function filters the issues, based on the content of the issue body
/// and criteria described in the design documentation, and returns a vector
/// that only contains issues with valid data to be processed.
///
/// ## Criteria
/// Based on the design documentation, the following standard must be met for an issue to be valid:
///
/// 2. The issue body *can* be written in Markdown and *can* contain anything that can be written in Markdown.
/// 3. The issue body *must* contain a **fenced code block**, for which:
///     1. *contains* the data for the corresponding friend link entry.
///     2. *must* be set to `json` language.
///     3. *must* contain a valid JSON object containing data for the friend link entry.
///     4. *should* be the **only** code block in the issue body.
///     5. *must* be preceded by a `<!-- DATA_START -->` comment.
///     6. *must* be followed by a `<!-- DATA_END -->` comment.
///     7. *must* be the only Markdown content between the `<!-- DATA_START -->` and `<!-- DATA_END -->` comments.
///     8. No other `<!-- DATA_START -->` or `<!-- DATA_END -->` comments can exist in the issue body.
///
/// *(some other parts are not included since they are not relevant to this function)*
///
/// ## Arguments
/// - `issues`: A vector of `Issue` structs representing the issues to be filtered.
///
/// ## Returns
/// A vector of `Issue` structs that contains all valid issues based on the criteria.
fn get_all_valid_issues(issues: Vec<github_api_responses::Issue>) -> Vec<github_api_responses::Issue> {
    issues.into_iter()
        .filter(|issue| {
            println!("Checking issue, ID: {}", issue.id);

            let body = issue.body.clone();
            let data_start = "<!-- DATA_START -->";
            let data_end = "<!-- DATA_END -->";
            let code_block_start = "```json";
            let code_block_end = "```";

            // Find the index of data start and end comments.
            let data_start_index = body.find(data_start);
            let data_end_index = body.find(data_end);

            // Check if the comments exist.
            if data_start_index.is_none() || data_end_index.is_none() {
                println!("Missing DATA_START or DATA_END comment.");
                return false;
            }
            let data_start_index = data_start_index.unwrap();
            let data_end_index = data_end_index.unwrap();

            // Check if the comments are in the correct order.
            if data_start_index > data_end_index {
                println!("DATA_START comment is after DATA_END comment.");
                return false;
            }
            // Check if the comments are the only pair in the body.
            if body.matches(data_start).count() != 1 || body.matches(data_end).count() != 1 {
                println!("Multiple DATA_START or DATA_END comments found.");
                return false;
            }

            // Extract the data section between the comments.
            let data_section = &body[data_start_index + data_start.len()..data_end_index].trim();

            // Check if only a code block exists in the data section.
            if !(data_section.starts_with(code_block_start) && data_section.ends_with(code_block_end)) {
                println!("Other Markdown content found in the data section.");
                return false;
            }
            // Check if the code block is the only one in the data section.
            // The check is `data_section.matches(code_block_end).count() != 2` is done as the bit "```" is also included in the start of the code block.
            if data_section.matches(code_block_start).count() != 1 || data_section.matches(code_block_end).count() != 2 {
                println!("Multiple code blocks (or other Markdown content) found in the data section.");
                return false;
            }

            // Extract the code block content.
            let code_block = &data_section[code_block_start.len()..data_section.len() - code_block_end.len()];

            // Check if the code block content is valid JSON.
            serde_json::from_str::<serde_json::Value>(code_block).is_ok()
        })
        .collect()
}

/// This function returns the list of issue that is active
/// depending on the provided label that identifies the active issues.
///
/// ## Arguments
/// - `label`: The name of the label that identifies the active issues.
/// - `issues`: A vector of `Issue` structs representing the issues to be filtered.
///
/// ## Returns
/// A vector of `Issue` structs that contains all active issues (i.e. with the specified label).
fn get_all_active_entries(label: String, issues: Vec<github_api_responses::Issue>) -> Vec<github_api_responses::Issue> {
    issues.into_iter()
        .filter(|issue| issue.labels.iter().any(|l| l.name == label))
        .collect()
}

/// This function converts the map between friend links groups and actual list of entries
/// into the needed JSON format for the output file.
///
/// ## Arguments
/// - `groups`: A reference to a vector of `GroupConfig` structs
///     that contains the necessary information about the link groups.
/// - `group_to_issue_map`: A reference to a `HashMap` that maps group labels to a vector of
///     `Issue` structs representing the friend links entries.
///
/// ## Returns
/// A `String` that contains the JSON representation of the grouped issues.
/// A sample structure can be seen in the design documentation.
///
fn group_to_issue_map_to_json(
    groups: &Vec<GroupConfig>,
    group_to_issue_map: &HashMap<String, Vec<github_api_responses::Issue>>,
) -> String {
    // Sample Structure:
    // [
    //   {
    //     "group": "LABEL_FOR_GROUP_1",
    //     "groupName": "Group 1",
    //     "groupDesc": "Description for Group 1",
    //     "entries": [
    //       {
    //         "name": "My Blog",
    //         "url": "https://myblog.com",
    //         "description": "A blog about my life and stuff.",
    //         "avatar": "https://myblog.com/avatar.png"
    //       },
    //       {
    //         "name": "My Other Blog",
    //         "url": "https://myotherblog.com",
    //         "description": "A blog about my other life and stuff.",
    //         "avatar": "https://myotherblog.com/avatar.png"
    //       },
    //       // ... other entries
    //     ]
    //   },
    //   {
    //     "group": "LABEL_FOR_GROUP_2",
    //     "groupName": "Group 2",
    //     "groupDesc": "Description for Group 2",
    //     "entries": [
    //       // ... entries for group 2
    //     ]
    //   },
    //   // ... other groups
    // ]

    // TODO: This `String::new()` is a placeholder.
    String::new()
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
    println!();

    // Call the function to get all issues from the GitHub repository.
    let issues = get_all_issues(&config).await;

    // Filter the issues to only get valid ones based on the specified criteria.
    let issues = get_all_valid_issues(issues);

    // Filter the entries to get only the active ones based on the specified label.
    let issues = get_all_active_entries(config.generation.label, issues);

    // Group the entries based on the groups defined in the configuration.
    let mut group_to_issue_map: HashMap<String, Vec<github_api_responses::Issue>> = config.groups.iter()
        .map(|group| (group.label.clone(), Vec::new()))
        .collect();
    // Process each issue.
    for issue in issues {
        // Check if the issue has any of the group labels.
        for group in &config.groups {
            if issue.labels.iter().any(|l| l.name == group.label) {
                // If it does, add the issue to the corresponding group.
                group_to_issue_map.entry(group.label.clone()).or_default().push(issue.clone());
            }
        }
    }
    // Print the grouped issues.
    println!("\nGrouped Issues:");
    for (group_label, issues) in &group_to_issue_map {
        println!("Group: {}", group_label);
        for issue in issues {
            println!("  - Issue ID: {}", issue.id);
            println!("    Issue Title: {}", issue.title);
            println!("    Issue URL: {}", issue.url);
        }
    }
}
