# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
As a pre-1.0 project (currently 0.x), breaking changes may occur between minor versions.
See [RELEASING.md](RELEASING.md) for the release process and migration guidance.

## [Unreleased]

## [0.1.4] - 2024-07-19

### Changed

- Overhaul: updated `package.json` to enhance npm compatibility
- Updated WASM target to `bundler` for wider npm ecosystem compatibility
- README updated with npm install instructions

### Fixed

- Fixed CI coverage reporting condition

## [0.1.3] - 2023-11-10

### Changed

- Updated WASM build target to `bundler`

## [0.1.2] - 2023-11-10

### Fixed

- Minor fixes and version bump

## [0.1.1] - 2023-11-10

### Fixed

- Fixed npm publish workflow

## [0.1.0] - 2023-11-10

### Added

- Initial release
- WASM bindings for rectangle dividing algorithm
- Published to crates.io and npm (`@kitsuyui/rectangle-dividing`)
- Core `dividing()` function with `rect`, `weights`, `aspect_ratio`, `vertical_first`, and `boustrophedron` parameters

## Known Issues

### `boustrophedron` parameter name typo (tracked separately)

The WASM `dividing()` function exposes a parameter named `boustrophedron`, which is a
misspelling of the correct term `boustrophedon` (alternating direction, like ox-turning
in ancient writing). The internal Rust implementation uses the correct spelling.

Because the WASM API is positional (not named), JavaScript callers are not affected by
the parameter name itself. However, documentation and any named bindings that reference
this parameter use the misspelled form.

This will be corrected in a future release. See [RELEASING.md](RELEASING.md) for the
planned migration approach.

[Unreleased]: https://github.com/kitsuyui/rust-rectangle-dividing/compare/v0.1.4...HEAD
[0.1.4]: https://github.com/kitsuyui/rust-rectangle-dividing/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/kitsuyui/rust-rectangle-dividing/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/kitsuyui/rust-rectangle-dividing/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/kitsuyui/rust-rectangle-dividing/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/kitsuyui/rust-rectangle-dividing/releases/tag/v0.1.0
