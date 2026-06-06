# Releasing

## Release Process

1. Update `version` in `Cargo.toml`
2. Run `cargo build` and `cargo test` to verify
3. Update `CHANGELOG.md`: move items from `[Unreleased]` to a new versioned section with the release date
4. Commit: `Bump version to X.Y.Z`
5. Push and open a PR; wait for CI to pass
6. Merge and tag: `git tag vX.Y.Z && git push origin vX.Y.Z`
7. GitHub Actions will publish to crates.io and npm automatically

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

## Known Upcoming Breaking Change: `boustrophedron` → `boustrophedon` rename

The WASM `dividing()` function has a parameter named `boustrophedron` (line 24 of
`src/wasm_binding.rs`). The correct English term is `boustrophedon`. The internal
Rust implementation (`src/dividing.rs`) already uses the correct spelling.

**Impact**: The WASM API is positional, so JavaScript callers passing the argument
by position are not affected. However, any code that relies on the Rust parameter
name (e.g. via `wasm-bindgen` named argument features, documentation generators,
or typed bindings) would break on rename.

**Planned fix**: The rename will happen in a future minor version bump. The
`CHANGELOG.md` will record the version in which this is fixed. No deprecation
shim is planned; callers should update call sites when upgrading past that version.
