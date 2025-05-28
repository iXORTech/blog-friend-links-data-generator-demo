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
mod link_entry;

use crate::config::GroupConfig;
use crate::link_entry::LinkEntry;
use config::Config;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use std::collections::HashMap;
use std::fs;

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
    let res = client
        .get(url)
        .header(
            USER_AGENT,
            "blog-friend-links-data-generator by iXOR Technology",
        )
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
/// A vector of `LinkEntry` structs that contains the data, representing the friend links entries,
/// retrieved from the valid issues.
fn get_all_valid_issues(issues: Vec<github_api_responses::Issue>) -> Vec<LinkEntry> {
    let mut entries: Vec<LinkEntry> = Vec::new();

    for issue in issues {
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
            continue;
        }
        let data_start_index = data_start_index.unwrap();
        let data_end_index = data_end_index.unwrap();

        // Check if the comments are in the correct order.
        if data_start_index > data_end_index {
            println!("DATA_START comment is after DATA_END comment.");
            continue;
        }
        // Check if the comments are the only pair in the body.
        if body.matches(data_start).count() != 1 || body.matches(data_end).count() != 1 {
            println!("Multiple DATA_START or DATA_END comments found.");
            continue;
        }

        // Extract the data section between the comments.
        let data_section = &body[data_start_index + data_start.len()..data_end_index].trim();

        // Check if only a code block exists in the data section.
        if !(data_section.starts_with(code_block_start) && data_section.ends_with(code_block_end)) {
            println!("Other Markdown content found in the data section.");
            continue;
        }
        // Check if the code block is the only one in the data section.
        // The check is `data_section.matches(code_block_end).count() != 2` is done as the bit "```" is also included in the start of the code block.
        if data_section.matches(code_block_start).count() != 1
            || data_section.matches(code_block_end).count() != 2
        {
            println!("Multiple code blocks (or other Markdown content) found in the data section.");
            continue;
        }

        // Extract the code block content.
        let code_block =
            &data_section[code_block_start.len()..data_section.len() - code_block_end.len()];

        // Check if the code block content is valid JSON.
        if !serde_json::from_str::<serde_json::Value>(code_block).is_ok() {
            println!("Invalid JSON in the code block.");
            continue;
        }

        // If all checks passed, create a `LinkEntry` from the issue data.
        let entry = LinkEntry {
            id: issue.id,
            labels: issue.labels.iter().map(|l| l.name.clone()).collect(),
            json_data: serde_json::from_str(code_block).expect("Failed to Parse JSON Data"),
            created_at: issue.created_at(),
            updated_at: issue.updated_at(),
        };

        // Add the entry to the list of entries.
        entries.push(entry);
    }

    entries
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
fn get_all_active_entries(label: String, issues: Vec<LinkEntry>) -> Vec<LinkEntry> {
    issues
        .into_iter()
        .filter(|issue| issue.labels.contains(&label))
        .collect()
}

/// This function converts the map between friend links groups and actual list of entries
/// into the needed JSON format for the output file.
///
/// ## Arguments
/// - `groups`: A reference to a vector of `GroupConfig` structs
///    that contains the necessary information about the link groups.
/// - `group_to_entry_map`: A reference to a `HashMap` that maps link entries (as a vector)
///    to their corresponding group labels.
///
/// ## Returns
/// The needed JSON structure for representing the generated data.
fn generate_json(
    groups: &Vec<GroupConfig>,
    group_to_entry_map: &HashMap<String, Vec<LinkEntry>>,
) -> Vec<serde_json::Value> {
    let mut json_data: Vec<serde_json::Value> = Vec::new();

    for group in groups {
        // Get the entries for the current group.
        if let Some(entries) = group_to_entry_map.get(&group.label) {
            // Create a JSON object for the group.
            let group_json = serde_json::json!({
                "group": group.label,
                "groupName": group.name,
                "groupDesc": group.description,
                "entries": entries.iter().map(|entry| entry.json_data.clone()).collect::<Vec<_>>()
            });
            // Add the group JSON to the list.
            json_data.push(group_json);
        }
    }

    json_data
}

#[tokio::main]
async fn main() {
    // Read the config.toml file and parse it.
    let config_file: String =
        fs::read_to_string("config.toml").expect("Failed to Read Configuration File");
    let config: Config = toml::from_str(&config_file).expect("Failed to Parse Configuration");

    println!("Github Token: {}", config.github.token);
    println!("Github Owner: {}", config.github.owner);
    println!("Github Repository: {}", config.github.repository);

    println!("Generation Label: {}", config.generation.label);
    println!(
        "Sort by Updated Time: {}",
        config.generation.sort_by_updated_time
    );

    println!("Groups:");
    for group in &config.groups {
        println!("  - Name: {}", group.name);
        println!("    Description: {}", group.description);
        println!("    Label: {}", group.label);
    }
    println!();

    // Filter the issues to only get valid ones based on the specified criteria.
    let entries = get_all_valid_issues(get_all_issues(&config).await);

    // Filter the entries to get only the active ones based on the specified label.
    let entries = get_all_active_entries(config.generation.label, entries);

    // Group the entries based on the groups defined in the configuration.
    let mut group_to_entry_map: HashMap<String, Vec<LinkEntry>> = config
        .groups
        .iter()
        .map(|group| (group.label.clone(), Vec::new()))
        .collect();
    // Process each issue.
    for entry in entries {
        // Check if the issue has any of the group labels.
        for group in &config.groups {
            if entry.labels.iter().any(|label| label == &group.label) {
                // If it does, add the issue to the corresponding group.
                group_to_entry_map
                    .entry(group.label.clone())
                    .or_default()
                    .push(entry.clone());
            }
        }
    }
    // Print the grouped issues.
    println!("\nGrouped Issues:");
    for (group_label, issues) in &group_to_entry_map {
        println!("Group: {}", group_label);
        for issue in issues {
            println!("  - Entry ID: {}", issue.id);
            println!("    Entry Data: {}", issue.json_data);
        }
    }

    // Generate the JSON output from the grouped issues.
    let json_output = generate_json(&config.groups, &group_to_entry_map);

    // Clean output directory if it exists.
    if fs::metadata("output").is_ok() {
        fs::remove_dir_all("output").expect("Failed to Remove Output Directory");
    }
    fs::create_dir_all("output").expect("Failed to Create Output Directory");

    // Write the JSON output to a file.
    fs::write(
        "output/linkData.json",
        serde_json::to_string_pretty(&json_output).unwrap(),
    )
    .expect("Failed to Write Output File");
}
