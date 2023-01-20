# cargo-no-dev-deps

[![crates.io](https://img.shields.io/crates/v/cargo-no-dev-deps?style=flat-square&logo=rust)](https://crates.io/crates/cargo-no-dev-deps)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)](#license)
[![rustc](https://img.shields.io/badge/rustc-1.60+-blue?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![build status](https://img.shields.io/github/actions/workflow/status/taiki-e/cargo-no-dev-deps/ci.yml?branch=main&style=flat-square&logo=github)](https://github.com/taiki-e/cargo-no-dev-deps/actions)

Cargo subcommand for running cargo without dev-dependencies.

This is an extraction of the [cargo-hack's `--no-dev-deps` flag](https://github.com/taiki-e/cargo-hack#--no-dev-deps) to be used as a stand-alone cargo subcommand.

- [Usage](#usage)
- [Installation](#installation)
- [Related Projects](#related-projects)
- [License](#license)

## Usage

<!-- readme-long-help:start -->
```console
$ cargo no-dev-deps --help
cargo-no-dev-deps

Cargo subcommand for running cargo without dev-dependencies.

USAGE:
    cargo no-dev-deps [CARGO_OPTIONS]
```
<!-- readme-long-help:end -->

To run `cargo check` without dev-deps:

```sh
cargo no-dev-deps check
```

**Note:** cargo-no-dev-deps modifies `Cargo.toml` and `Cargo.lock` while running and restores it when finished. Any changes you made to those files during running will not be preserved.

If you want exclude `publish = false` crates, you can exclude these crates by using `--no-private` flag.

```sh
cargo no-dev-deps --no-private check
```

This flag is more powerful than [cargo-hack's `--ignore-private` flag](https://github.com/taiki-e/cargo-hack#--ignore-private), which also prevents private crate from affecting dependency resolution.

## Installation

<!-- omit in toc -->
### From source

```sh
cargo +stable install cargo-no-dev-deps --locked
```

*Compiler support: requires rustc 1.60+*

cargo-no-dev-deps is usually runnable with Cargo versions older than the Rust version
required for installation (e.g., `cargo +1.59 no-dev-deps check`).

<!-- omit in toc -->
### From prebuilt binaries

You can download prebuilt binaries from the [Release page](https://github.com/taiki-e/cargo-no-dev-deps/releases).
Prebuilt binaries are available for macOS, Linux (gnu and musl), Windows (static executable), and FreeBSD.

<details>
<summary>Example of script to download cargo-no-dev-deps</summary>

```sh
# Get host target
host=$(rustc -Vv | grep host | sed 's/host: //')
# Download binary and install to $HOME/.cargo/bin
curl -LsSf https://github.com/taiki-e/cargo-no-dev-deps/releases/latest/download/cargo-no-dev-deps-$host.tar.gz | tar xzf - -C $HOME/.cargo/bin
```

</details>

<!-- omit in toc -->
### On GitHub Actions

You can use [taiki-e/install-action](https://github.com/taiki-e/install-action) to install prebuilt binaries on Linux, macOS, and Windows.
This makes the installation faster and may avoid the impact of [problems caused by upstream changes](https://github.com/tokio-rs/bytes/issues/506).

```yaml
- uses: taiki-e/install-action@cargo-no-dev-deps
```

<!-- omit in toc -->
### Via Homebrew

You can install [cargo-no-dev-deps using Homebrew tap on macOS and Linux](https://github.com/taiki-e/homebrew-tap/blob/HEAD/Formula/cargo-no-dev-deps.rb):

```sh
brew install taiki-e/tap/cargo-no-dev-deps
```

<!-- omit in toc -->
### Via Scoop (Windows)

You can install [cargo-no-dev-deps using Scoop](https://github.com/taiki-e/scoop-bucket/blob/HEAD/bucket/cargo-no-dev-deps.json):

```sh
scoop bucket add taiki-e https://github.com/taiki-e/scoop-bucket
scoop install cargo-no-dev-deps
```

<!-- omit in toc -->
### Via cargo-binstall

You can install cargo-no-dev-deps using [cargo-binstall](https://github.com/ryankurte/cargo-binstall):

```sh
cargo binstall cargo-no-dev-deps
```

## Related Projects

- [cargo-hack]: Cargo subcommand to provide various options useful for testing and continuous integration.
- [cargo-llvm-cov]: Cargo subcommand to easily use LLVM source-based code coverage.
- [cargo-minimal-versions]: Cargo subcommand for proper use of `-Z minimal-versions`.
- [cargo-config2]: Library to load and resolve Cargo configuration.

[cargo-config2]: https://github.com/taiki-e/cargo-config2
[cargo-hack]: https://github.com/taiki-e/cargo-hack
[cargo-llvm-cov]: https://github.com/taiki-e/cargo-llvm-cov
[cargo-minimal-versions]: https://github.com/taiki-e/cargo-minimal-versions

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
