# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

## [0.2.2] - 2023-02-28

- Fix compatibility with old cargo.

## [0.2.1] - 2023-01-24

- Update `toml_edit` to 0.18.

## [0.2.0] - 2023-01-20

- Add `--no-private` flag to exclude `publish = false` crates.

  This flag is more powerful than [cargo-hack's `--ignore-private` flag](https://github.com/taiki-e/cargo-hack#--ignore-private), which also prevents private crate from affecting lockfile and metadata.

- Restore `Cargo.lock` after run to match behavior with [cargo-minimal-versions](https://github.com/taiki-e/cargo-minimal-versions).

- Documentation improvements.

## [0.1.0] - 2023-01-11

Initial release

[Unreleased]: https://github.com/taiki-e/cargo-no-dev-deps/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/taiki-e/cargo-no-dev-deps/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/taiki-e/cargo-no-dev-deps/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/taiki-e/cargo-no-dev-deps/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/taiki-e/cargo-no-dev-deps/releases/tag/v0.1.0
