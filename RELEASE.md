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

Releases are triggered automatically when a version tag is pushed to GitHub.

The `publish.yml` workflow will:

1. Validate the release version
2. Update the `Cargo.toml` and `Cargo.lock` versions
3. Commit and push the updated files
4. Publish the crate to crates.io

---

## Creating a Release

### 1. Create an annotated tag

The tag must follow the crate version format:

``` git tag -a vX.Y.Z -m "[DESCRIPTION]" ```

### 2. Push the tag to GitHub

``` git push --tags ```

---

## Notes

- The tag version must follow semantic versioning (`vMAJOR.MINOR.PATCH`)
- The workflow only triggers for tags starting with `v`
- The current link of the test package is ```https://crates.io/crates/test-debtlint```
