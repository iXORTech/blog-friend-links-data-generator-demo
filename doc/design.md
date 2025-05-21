# Project Design

A lot of blog and/or static site generators uses JSON format to store friend links data (name, url, description, etc.),
and it is usually a hassle to manage all the entries manually. Managing all the entries with GitHub Issues (i.e. every
entry is a GitHub Issue and all the information are contained in the issue body) could be then a solution.

This project aims to provide a full working package for this purpose, including:

- A Rust script to generate the JSON file from GitHub Issues.
- A GitHub Action to run the script automatically on a schedule and/or on issue edit events.
- A GitHub Issue template that follows the format required by the script.

## Format of the GitHub Issue

The GitHub Issue should follow a specific format in order to be parsed correctly by the script. The general standard and
structure for this project is as follows:

1. The issue body is the **only** part to be read and processed by the script.
2. The issue body *can* be written in Markdown and *can* contain anything that can be written in Markdown.
3. The issue body *must* contain a **fenced code block**, for which:
    1. *contains* the data for the corresponding friend link entry.
    2. *must* be set to `json` language.
    3. *must* contain a valid JSON object containing data for the friend link entry.
    4. *should* be the **only** code block in the issue body.
    5. *must* be preceded by a `<!-- DATA_START -->` comment.
    6. *must* be followed by a `<!-- DATA_END -->` comment.
    7. *must* be the only Markdown content between the `<!-- DATA_START -->` and `<!-- DATA_END -->` comments.
    8. No other `<!-- DATA_START -->` or `<!-- DATA_END -->` comments can exist in the issue body.
4. The `generation.label` configuration defines the label that is used to identify the issues that contains data to be included in the generated data.
5. The `generation.groups` configurations defines a list of groups that categorizes the data. Each entry of the list, a string, is also used as the label to identify the issues that contains data to be included in that group.

As sample of the JSON data code block would be:

```json
{
    "name": "My Blog",
    "url": "https://myblog.com",
    "description": "A blog about my life and stuff.",
    "avatar": "https://myblog.com/avatar.png",
}
```

## Script

This script is written in Rust and uses [rust-script](https://rust-script.org/) to run it as a script without the need
to compile it ahead of time. Other libraries used are:

- [reqwest](https://crates.io/crates/reqwest): for making HTTP requests (e.g. to the GitHub API).
- [serde](https://crates.io/crates/serde): for serializing and deserializing TOML and JSON data.
- [serde_json](https://crates.io/crates/serde_json): for handling JSON serialization and deserialization with Serde.
- [tokio](https://crates.io/crates/tokio): for async runtime.
- [toml](https://crates.io/crates/toml): for configuration parsing.
- *list to be completed*.

And the processing logic is as follows:

- Read the configuration file and get the necessary GitHub and Data Generator configurations.
- Through the GitHub API, get the list of issues from the repository.
- Read each issue, and check if it is labeled with the `generation.label` label.
    - If it is: check if it contains the `<!-- DATA_START -->` and `<!-- DATA_END -->` comments.
        - If it does: check if it contains a code block with language set to `json` language and contains a valid JSON object.
            - If it does: parse the JSON object and add it to the list of entries.
            - If it does not: no further action is taken for this issue.
        - If it does not: no further action is taken for this issue.
    - If it is not: no further action is taken for this issue.
