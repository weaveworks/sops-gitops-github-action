[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fweaveworks%2Fsops-gitops-github-action.svg?type=shield&issueType=license)](https://app.fossa.com/projects/git%2Bgithub.com%2Fweaveworks%2Fsops-gitops-github-action?ref=badge_shield&issueType=license)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fweaveworks%2Fsops-gitops-github-action.svg?type=shield&issueType=security)](https://app.fossa.com/projects/git%2Bgithub.com%2Fweaveworks%2Fsops-gitops-github-action?ref=badge_shield&issueType=security)

# GitHub Action: GitOps SOPS Secret Manager

## Status

This tool is in active development and is not considered ready for use in production environments or even as a GitHub Action. 
We are actively working on this tool and will update this README when it is ready for use.

## Overview

This GitHub Action simplifies managing encrypted secrets using SOPS (Secrets OPerationS) and GPG. The action automates the encryption and decryption of secrets while maintaining a secure configuration, fetching public keys for team members directly from the GitHub API.

## Features

- Fetches team and contributor public GPG keys directly from the GitHub API.
- Manages encryption and decryption of secrets using SOPS.
- Automatically updates `.sops.yaml` configuration with valid GPG fingerprints.
- Compatible with repositories containing sensitive information requiring secure handling.

## Prerequisites

1. **GPG Keys**:
    - Public keys for all team members.
    - Private and public keys for the application.
2. **GitHub API Token**:
    - Required to fetch contributor/team public keys. Store this token as a secret (e.g., `GH_TOKEN`) in your repository.
3. **Rust Environment**:
    - This action is implemented as a Rust binary packaged in a Docker container.

## Usage

### Inputs

| Input Name           | Description                                           | Required | Default |
|----------------------|-------------------------------------------------------|----------|---------|
| `gpg_private_key`    | Base64-encoded private GPG key for the application.   | Yes      |         |
| `github_token`       | GitHub token for fetching team and contributor keys.  | Yes      |         |
| `workspace`          | Directory containing secrets to manage.              | No       | `.`     |
| `public_key_path`    | Path to the application public GPG key.              | No       | `./.github/gpg.pub` |

### Outputs

| Output Name          | Description                                           |
|----------------------|-------------------------------------------------------|
| `message`            | Summary message of the action's execution.           |

### Example Workflow

```yaml
name: Manage Secrets with SOPS

on:
  push:
    branches:
      - main

jobs:
  manage-secrets:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout repository
        uses: actions/checkout@v3

      - name: Manage SOPS secrets
        uses: your-org/your-repo@v1
        with:
          gpg_private_key: ${{ secrets.GPG_KEY }}
          github_token: ${{ secrets.GH_TOKEN }}
          workspace: ./secrets
          public_key_path: ./.github/gpg.pub
```

### How It Works

1. Fetch GPG Keys:

- The action retrieves public keys from the GitHub API for all contributors and team members in the repository.
- Adds the application’s public GPG key to the list of keys.

2. Update .sops.yaml:

- Populates .sops.yaml with the GPG fingerprints of all valid keys.

3. Encrypt/Decrypt Secrets:

- Locates all files containing secrets in the specified workspace.
- Encrypts or updates encryption keys for these files.

### Notes
- Ensure all required secrets (e.g., GPG_KEY) are stored securely in the repository settings.
- This action works seamlessly with YAML, JSON not currently supported.
- For additional debugging, set the DEBUG environment variable in your workflow to true.

### Development

Building the Docker Image
```bash
Copy code
docker build -t rust-sops-action .
```

### Running Locally
```bash
Copy code
docker run -e INPUT_GPG_PRIVATE_KEY="<base64-key>" \
-e INPUT_GITHUB_TOKEN="<github-token>" \
rust-sops-action
```

###
Dependency report for this GitHub Action is available [here](https://app.fossa.com/reports/7b1b1b3b-1b1b-4b1b-8b1b-1b1b1b1b1b1b).

License
This action is licensed under the MIT License.
