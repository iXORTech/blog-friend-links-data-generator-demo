/// The structure of an individual link entry, retrieved from the GitHub issue,
/// to be added to the generated data file.
#[derive(Clone)]
pub(crate) struct LinkEntry {
    /// The unique identifier for the link entry, same as the ID of the GitHub issue.
    pub(crate) id: usize,
    /// The list of GitHub labels associated with the issue.
    pub(crate) labels: Vec<String>,
    /// The JSON data of the link entry contained in the issue body.
    pub(crate) json_data: serde_json::Value,
    /// The time when the issue for submitting the link entry was created.
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
    /// The time when the issue for submitting the link entry was last updated.
    pub(crate) updated_at: chrono::DateTime<chrono::Utc>,
}
