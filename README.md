# cargo-no-dev-deps

[![crates.io](https://img.shields.io/crates/v/cargo-no-dev-deps?style=flat-square&logo=rust)](https://crates.io/crates/cargo-no-dev-deps)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)](#license)
[![github actions](https://img.shields.io/github/actions/workflow/status/taiki-e/cargo-no-dev-deps/ci.yml?branch=main&style=flat-square&logo=github)](https://github.com/taiki-e/cargo-no-dev-deps/actions)

Cargo subcommand for running cargo without dev-dependencies.

This is an extraction of the [cargo-hack's `--no-dev-deps` flag](https://github.com/taiki-e/cargo-hack#--no-dev-deps) to be used as a stand-alone cargo subcommand.

- [Usage](#usage)
- [Installation](#installation)
- [Related Projects](#related-projects)
- [License](#license)

## Usage

<details>
<summary>Complete list of options (click to show)</summary>

<!-- readme-long-help:start -->
```console
$ cargo no-dev-deps --help
cargo-no-dev-deps

Cargo subcommand for running cargo without dev-dependencies.

USAGE:
    cargo no-dev-deps <CARGO_SUBCOMMAND> [OPTIONS] [CARGO_OPTIONS]

OPTIONS:
        --no-private    Perform without `publish = false` crates

CARGO_SUBCOMMANDS:
    build
    check
    ...
```
<!-- readme-long-help:end -->

</details>

To run `cargo check` without dev-deps:

```sh
cargo no-dev-deps check
```

> [!NOTE]
> cargo-no-dev-deps modifies `Cargo.toml` and `Cargo.lock` while running and restores it when finished. Any changes you made to those files during running will not be preserved.

If you want exclude `publish = false` crates, you can exclude these crates by using `--no-private` flag.

```sh
cargo no-dev-deps --no-private check
```

This flag is more powerful than [cargo-hack's `--ignore-private` flag](https://github.com/taiki-e/cargo-hack#--ignore-private), because this also prevents private crates from affecting lockfile and metadata.

## Installation

<!-- omit in toc -->
### From source

```sh
cargo +stable install cargo-no-dev-deps --locked
```

Currently, installing cargo-no-dev-deps requires rustc 1.85+.

cargo-no-dev-deps is usually runnable with Cargo versions older than the Rust version
required for installation (e.g., `cargo +1.59 no-dev-deps check`).

<!-- omit in toc -->
### From prebuilt binaries

You can download prebuilt binaries from the [Release page](https://github.com/taiki-e/cargo-no-dev-deps/releases).
Prebuilt binaries are available for Linux (x86_64 gnu/musl, aarch64 gnu/musl, powerpc64le gnu/musl, riscv64gc gnu/musl, and s390x gnu, musl binaries are static executable), macOS (x86_64, aarch64, and universal), Windows (x86_64 and aarch64, static executable), FreeBSD (x86_64), and illumos (x86_64).
All releases are [immutable](https://docs.github.com/en/code-security/concepts/supply-chain-security/immutable-releases) and releases since 0.2.23 publish [artifact attestations](https://docs.github.com/en/actions/concepts/security/artifact-attestations).

<details>
<summary>Example of script to install from the Release page with verifications (click to show)</summary>

```sh
# Get host target.
host=$(rustc -vV | grep '^host:' | cut -d' ' -f2)
# Download binary.
curl --proto '=https' --tlsv1.2 -fsSL -o cargo-no-dev-deps.tar.gz "https://github.com/taiki-e/cargo-no-dev-deps/releases/latest/download/cargo-no-dev-deps-${host}.tar.gz"
# Verify release attestations.
gh release -R https://github.com/taiki-e/cargo-no-dev-deps verify-asset cargo-no-dev-deps.tar.gz
# Verify artifact attestations.
gh attestation verify --owner taiki-e cargo-no-dev-deps.tar.gz
# Install to $CARGO_HOME/bin (or $HOME/.cargo/bin if CARGO_HOME is unset).
tar xf cargo-no-dev-deps.tar.gz -C "${CARGO_HOME:-"$HOME/.cargo"}"/bin
# Remove archive.
rm cargo-no-dev-deps.tar.gz
```

`gh release -R .. verify-asset` should output messages like the following (`<hash256>`/`<hash1>`/`<version>` contains the actual SHA-256 digest of the downloaded file / the actual SHA-1 digest of tag / the actual version number):

```text
Calculated digest for cargo-no-dev-deps.tar.gz: sha256:<hash256>
Resolved tag v<version> to sha1:<hash1>
Loaded attestation from GitHub API

✓ Verification succeeded! cargo-no-dev-deps.tar.gz is present in release v<version>
```

`gh attestation verify` should output messages like the following (`<hash256>` contains the actual SHA-256 digest of the downloaded file):

```text
Loaded digest sha256:<hash256> for file://cargo-no-dev-deps.tar.gz
Loaded 1 attestation from GitHub API

The following policy criteria will be enforced:
- Predicate type must match:................ https://slsa.dev/provenance/v1
- Source Repository Owner URI must match:... https://github.com/taiki-e
- Subject Alternative Name must match regex: (?i)^https://github.com/taiki-e/
- OIDC Issuer must match:................... https://token.actions.githubusercontent.com

✓ Verification succeeded!

The following 1 attestation matched the policy criteria

- Attestation #1
  - Build repo:..... taiki-e/cargo-no-dev-deps
  - Build workflow:. .github/workflows/release.yml@refs/heads/main
  - Signer repo:.... taiki-e/github-actions
  - Signer workflow: .github/workflows/rust-release.yml@refs/heads/main
```

</details>

<details>
<summary>Example of script to install from the Release page without verification (click to show)</summary>

```sh
# Get host target.
host=$(rustc -vV | grep '^host:' | cut -d' ' -f2)
# Download binary and install to $CARGO_HOME/bin (or $HOME/.cargo/bin if CARGO_HOME is unset).
curl --proto '=https' --tlsv1.2 -fsSL "https://github.com/taiki-e/cargo-no-dev-deps/releases/latest/download/cargo-no-dev-deps-$host.tar.gz" \
  | tar xzf - -C "${CARGO_HOME:-"$HOME/.cargo"}"/bin
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

You can install cargo-no-dev-deps from the [Homebrew tap maintained by us](https://github.com/taiki-e/homebrew-tap/blob/HEAD/Formula/cargo-no-dev-deps.rb) (x86_64/AArch64 macOS, x86_64/AArch64 Linux):

```sh
brew install taiki-e/tap/cargo-no-dev-deps
```

<!-- omit in toc -->
### Via Scoop (Windows)

You can install cargo-no-dev-deps from the [Scoop bucket maintained by us](https://github.com/taiki-e/scoop-bucket/blob/HEAD/bucket/cargo-no-dev-deps.json):

```sh
scoop bucket add taiki-e https://github.com/taiki-e/scoop-bucket
scoop install cargo-no-dev-deps
```

<!-- omit in toc -->
### Via cargo-binstall

You can install cargo-no-dev-deps using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall):

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
