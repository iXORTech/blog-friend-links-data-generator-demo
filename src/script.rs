#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! serde = { version = "1.0.219", features = ["derive"] }  # Serialization/Deserialization
//! toml = "0.8.22" # TOML Parsing
//! ```

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    github: GithubConfig,
    generation: GenerationConfig
}

#[derive(Deserialize)]
struct GithubConfig {
    token: String,
    owner: String,
    repository: String,
}

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
