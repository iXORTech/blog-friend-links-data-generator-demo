use serde::Deserialize;

/// The structure of the script configuration.
///
/// It contains two parts:
/// - `github`: Configuration for GitHub API access.
/// - `generation`: Configuration for the data generation process.
#[derive(Deserialize)]
pub(crate) struct Config {
    pub github: GithubConfig,
    pub generation: GenerationConfig
}

/// The structure of the GitHub configuration.
///
/// It contains:
/// - `token`: The GitHub API access token.
/// - `owner`: The owner of the GitHub repository where issues to be processed are located.
/// - `repository`: The name of the GitHub repository where issues to be processed are located.
#[derive(Deserialize)]
pub(crate) struct GithubConfig {
    pub token: String,
    pub owner: String,
    pub repository: String,
}

/// The structure of the data generation configuration.
///
/// It contains:
/// - `label`: The label added to the issues to be included in the generated data.
/// - `groups`: The groups that separate issues and generated data into different categories.
/// - `sort_by_updated_time`: Whether to sort the issues by their updated time or creation time.
#[derive(Deserialize)]
pub(crate) struct GenerationConfig {
    pub label: String,
    pub groups: Vec<String>,
    pub sort_by_updated_time: bool,
}
