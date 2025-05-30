use serde::Deserialize;

/// The structure of the script configuration.
///
/// It contains two parts:
/// - `github`: Configuration for GitHub API access.
/// - `generation`: Configuration for the data generation process.
/// - `groups`: Configuration for the groups that separate issues and generated data into different categories.
#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) github: GithubConfig,
    pub(crate) generation: GenerationConfig,
    pub(crate) groups: Vec<GroupConfig>,
}

/// The structure of the GitHub configuration.
///
/// It contains:
/// - `owner`: The owner of the GitHub repository where issues to be processed are located.
/// - `repository`: The name of the GitHub repository where issues to be processed are located.
#[derive(Deserialize)]
pub(crate) struct GithubConfig {
    pub(crate) owner: String,
    pub(crate) repository: String,
}

/// The structure of the data generation configuration.
///
/// It contains:
/// - `label`: The label added to the issues to be included in the generated data.
/// - `sort_by_updated_time`: Whether to sort the issues by their updated time or creation time.
#[derive(Deserialize)]
pub(crate) struct GenerationConfig {
    pub(crate) label: String,
    pub(crate) sort_by_updated_time: bool,
}

/// The structure of a group configuration.
///
/// It contains:
/// - `name`: The name of the group.
/// - `description`: The description of the group.
/// - `label`: The label added to the issues to be included in this group.
#[derive(Deserialize)]
pub(crate) struct GroupConfig {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) label: String,
}
