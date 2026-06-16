# Release

## GitHub Repository Configuration

## Prerequisite

1. Create a crates.io API token with the following permissions:
    
    - ```publish-new```
    - ```publish-update```

2. Set the API token in a github secret named ```CARGO_REGISTRY_TOKEN```.
The workflow uses this token to authenticate and publish the crate during the release pipeline.

---

## Release Workflow

The `publish.yml` workflow will publish the crate to crates.io and validate is the version is valid.

---

## Creating a Release

### 1. Updates the package version

1. Update the Cargo.toml and Cargo.lock file by doing ``` cargo set-version v.X.Y.Z ```.
2. Commit the changes: ```git commit -m "chore: release X.Y.Z" -m "Release-As: X.Y.Z"```.
3. Push the commit to the remote repository: ```git push```.

---