# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

- Distribute prebuilt binary for x86_64 illumos.

## [0.2.12] - 2024-03-10

- Pin `ctrlc` to fix [build error on macOS](https://github.com/Detegr/rust-ctrlc/pull/116).

## [0.2.11] - 2024-02-10

- Update `toml_edit` to 0.22.

## [0.2.10] - 2024-01-24

- Fix "No such file or directory" error when `--no-private` flag is used with the workspace that the `members` field contains glob.

## [0.2.9] - 2023-12-16

- Remove dependency on `is-terminal`.

## [0.2.8] - 2023-12-05

- Update `toml_edit` to 0.21.

## [0.2.7] - 2023-09-11

- Remove dependency on `slab`, `shell-escape`, and `fs-err`.

## [0.2.6] - 2023-09-10

- Fix regression on `--no-private` flag with virtual workspace, introduced in 0.2.5.

## [0.2.5] - 2023-09-09

- Improve support for very old Cargo (pre-1.39).

- Remove dependency on `cargo_metadata`.

## [0.2.4] - 2023-08-28

- Fix bug in `--no-private` flag on Windows.

## [0.2.3] - 2023-07-28

- Update `cargo_metadata` to 0.17.

## [0.2.2] - 2023-02-28

- Fix compatibility with old cargo.

- Update `toml_edit` to 0.19.

## [0.2.1] - 2023-01-24

- Update `toml_edit` to 0.18.

## [0.2.0] - 2023-01-20

- Add `--no-private` flag to exclude `publish = false` crates.

  This flag is more powerful than [cargo-hack's `--ignore-private` flag](https://github.com/taiki-e/cargo-hack#--ignore-private), because this also prevents private crates from affecting lockfile and metadata.

- Restore `Cargo.lock` after run to match behavior with [cargo-minimal-versions](https://github.com/taiki-e/cargo-minimal-versions).

- Documentation improvements.

## [0.1.0] - 2023-01-11

Initial release

[Unreleased]: https://github.com/taiki-e/cargo-no-dev-deps/compare/v0.2.12...HEAD
[0.2.12]: https://github.com/taiki-e/cargo-no-dev-deps/compare/v0.2.11...v0.2.12
[0.2.11]: https://github.com/taiki-e/cargo-no-dev-deps/compare/v0.2.10...v0.2.11
[0.2.10]: https://github.com/taiki-e/cargo-no-dev-deps/compare/v0.2.9...v0.2.10
[0.2.9]: https://github.com/taiki-e/cargo-no-dev-deps/compare/v0.2.8...v0.2.9
[0.2.8]: https://github.com/taiki-e/cargo-no-dev-deps/compare/v0.2.7...v0.2.8
[0.2.7]: https://github.com/taiki-e/cargo-no-dev-deps/compare/v0.2.6...v0.2.7
[0.2.6]: https://github.com/taiki-e/cargo-no-dev-deps/compare/v0.2.5...v0.2.6
[0.2.5]: https://github.com/taiki-e/cargo-no-dev-deps/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/taiki-e/cargo-no-dev-deps/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/taiki-e/cargo-no-dev-deps/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/taiki-e/cargo-no-dev-deps/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/taiki-e/cargo-no-dev-deps/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/taiki-e/cargo-no-dev-deps/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/taiki-e/cargo-no-dev-deps/releases/tag/v0.1.0
