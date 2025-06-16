<h1 align="center">blog-friend-links-data-generator</h1>
<p align="center">A Rust Script used to generate blog friend links data (in JSON format) from GitHub Issue.</p>

> Why Rust Script? Because I can. Also an experimental project to learn Rust and play around with [rust-script](https://rust-script.org/).

## 📜 Features

Easily manage friend links and recommended websites data for your blog or website using GitHub issues. This project provides a set of tools to generate and maintain data file (in both JSON and JavaScript object formats) generated from GitHub issues.

## ⚙️ Requirements

- GitHub & GitHub Action
- Rust (1.74 or later) and Cargo
- [rust-script](https://rust-script.org/)

## 📥 Installation

1. Fork this repository to your GitHub account.
2. (optional) If needed, change the `label-commenter-config.yml` to customize the automated response when respective labels are added to issues. This file is located in the `.github/configs` directory of the repository.
3. Setup the configuration file following the instructions below:
   1. Copy the `src/config.toml` sample configuration file to the root of your repository.
   2. Edit the `config.toml` file so that:
       - The `owner` field under `[github]` is set to the owner (usually your GitHub username) of your fork.
       - The `repo` field under `[github]` is set to the name of your fork.
       - The `label` field under `[generation]` is set to the label you want to identify active issues. Only the entries contained in active issues (i.e. issues with this label) will be processed and added to the generated data.
       - The `sort_by_updated_time` should be set to `true` if you want the links to be sorted by their last updated time, or `false` if you want them sorted by creation time of the issue.
       - Any arbitrary number of `[[groups]]` that defines the groups used to categorize the links. Each group should have a `name`, a `description`, and a `label` field. The `label` field is used to identify the entries that belong to this group. The `name` and `description` fields are used to generate the data file.
   3. Comment out the `config.toml` line in the `.gitignore` to allow the configuration file to be added to your repository.
   4. Commit and push the changes to your repository.
4. Make sure that `Generate Friend Links Data` and `Label Commenter` actions are enabled in your repository. Also make sure that the workflow permission is set to `Read and write permissions` in the repository settings to allow the action to use the workflow GITHUB_TOKEN to perform the necessary operations.

### Friend Links Data Auto Sync (optional)

If you want your website to automatically update the friend links data once the new data is generated, following the instructions below:

> [!IMPORTANT]
> Doing this requires you to have necessary knowledge about git submodules and GitHub Actions.

1. Manually trigger the `Generate Friend Links Data` action once by adding an active issue or manually triggering the action from the Actions tab in your link data repository.
2. Remove the necessary directories containing the link data file from your website repository (if needed), and add the `data` branch of the link data repository as a submodule of your website repository under the directory you want.
3. Add the following GitHub Action workflow file to your website repository under `.github/workflows/sync-friend-links-data.yml` (make sure to replace `YOUR_SUBMODULE_DIRECTORY` with the directory where the submodule is located), commit and push the changes:
    
```yaml
name: 'Sync Friend Links Data'
 
on:
   # Scheduled to run at 00:00 UTC every day
   schedule:
     - cron: "0 0 * * *"
   # Manually Trigger
   workflow_dispatch:
   # Link data repository dispatch trigger
   repository_dispatch:
     types: [sync-friend-links-data]
 
jobs:
   sync:
     name: 'Submodules Sync'
     runs-on: ubuntu-latest
     steps:
     - name: Checkout
       uses: actions/checkout@v4
     - name: Git Submodule Update
       run: |
         git pull --recurse-submodules
         git submodule update --init --remote YOUR_SUBMODULE_DIRECTORY
     - name: Commit Update
       run: |
         git config --global user.name 'github-actions[bot]'
         git config --global user.email 'github-actions[bot]@users.noreply.github.com'
         git remote set-url origin https://x-access-token:${{ secrets.GITHUB_TOKEN }}@github.com/${{ github.repository }}
         git commit -am "deploy: Sync Friend Link Data" && git push || echo "No Changes to Commit"
```
   
4. Create a new GitHub token with `repo` and `workflow` permission and add it to your link data repository as a secret named `CI_TOKEN`.
5. Add your website repository and your submodule path as `WEBSITE_REPO` and `SUBMODULE_PATH` in your link data repository's variable section (Settings > Secrets and variables > Actions > Variables) respectively.
6. Add the following step to the `Generate Friend Links Data` action workflow in your link data repository's workflow file:
```yaml
 - name: Sync Friend Links Data
   run: |
     curl -X POST \
       -H "Accept: application/vnd.github+json" \
       -H "Authorization: Bearer ${{ secrets.CI_TOKEN }}" \
       -H "X-GitHub-Api-Version: 2022-11-28" \
       https://api.github.com/repos/${{ vars.WEBSITE_REPO }}/dispatches \
       -d "{\"event_type\": \"sync-friend-links-data\", \"client_payload\": {\"submodule\": \"${{ vars.SUBMODULE_PATH }}\"}}"
```

Now, whenever the `Generate Friend Links Data` action is triggered, it will also trigger the `Sync Friend Links Data` action in your website repository to update the friend links data submodule, and your website should always be kept up-to-date with the latest friend links data.

### Update the Project

To update the project, simply use the "Sync fork" button on the GitHub repository page to pull the latest changes from the original repository. This will update your fork with the latest changes and shouldn't require any additional steps.

## 🛠️ Usage

*TODO*
