// This file contains the data structures used to deserialize the JSON responses from the GitHub API.

use serde::Deserialize;

/// The structure of an individual issue in the response.
///
/// Note that this struct only contains the fields needed for the script to work,
/// and not all fields in the response data.
///
/// See: https://docs.github.com/en/rest/issues/issues?apiVersion=2022-11-28#list-repository-issues
#[derive(Deserialize, Debug)]
pub(crate) struct Issue {
    pub(crate) id: usize,
    pub(crate) url: String,
    pub(crate) number: usize,
    pub(crate) state: String,
    pub(crate) title: String,
    pub(crate) body: String,
    pub(crate) labels: Vec<Label>,
    pub(crate) closed_at: Option<String>,
    pub(crate) created_at: String,
    pub(crate) updated_at: String,
}

/// The structure of a label in the issue.
///
/// Note that this struct only contains the fields needed for the script to work,
/// and not all fields in the response data.
///
/// See: https://docs.github.com/en/rest/issues/issues?apiVersion=2022-11-28#list-repository-issues
#[derive(Deserialize, Debug)]
pub(crate) struct Label {
    pub(crate) id: usize,
    pub(crate) name: String,
    pub(crate) description: String,
}
