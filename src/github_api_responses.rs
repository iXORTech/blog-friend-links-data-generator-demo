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

impl Issue {
    /// Returns the closed_at date of the issue as a DateTime object.
    /// If the issue is not closed, it returns None.
    pub(crate) fn closed_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.closed_at
            .as_ref()
            .and_then(|date_str| chrono::DateTime::parse_from_rfc3339(date_str).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc))
    }

    /// Returns the created_at date of the issue as a DateTime object.
    pub(crate) fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::parse_from_rfc3339(&self.created_at)
            .expect("Failed to Parse created_at Date")
            .with_timezone(&chrono::Utc)
    }

    /// Returns the updated_at date of the issue as a DateTime object.
    pub(crate) fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::parse_from_rfc3339(&self.updated_at)
            .expect("Failed to Parse updated_at Date")
            .with_timezone(&chrono::Utc)
    }
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
