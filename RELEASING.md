# Releasing

## Release Process

1. Update `version` in `Cargo.toml`
2. Run `cargo build` and `cargo test` to verify
3. Update `CHANGELOG.md`: move items from `[Unreleased]` to a new versioned section with the release date
4. Commit: `Bump version to X.Y.Z`
5. Push and open a PR; wait for CI to pass
6. Merge the PR, then create a GitHub Release for `vX.Y.Z` targeting the merged commit.
7. Verify both GitHub Actions publishing workflows and the resulting crates.io and npm package versions.

## Versioning Policy

This project follows [Semantic Versioning](https://semver.org/). While the version is
0.x (pre-1.0), breaking changes to the public API may occur in minor version bumps.
Breaking changes will always be documented in `CHANGELOG.md`.

## Breaking Changes

When making a breaking change:

1. Add a `### Breaking` section in the `[Unreleased]` block in `CHANGELOG.md`
2. Describe the old behavior, the new behavior, and a migration path
3. If a transition period is needed, introduce the new API alongside the old one and
   mark the old one as deprecated before removing it in a subsequent release
