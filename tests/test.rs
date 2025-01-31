// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg(not(miri))] // Miri doesn't support file with non-default mode: https://github.com/rust-lang/miri/pull/2720

use std::{ffi::OsStr, path::Path, process::Command};

use test_helper::cli::CommandExt as _;

fn cargo_no_dev_deps<O: AsRef<OsStr>>(args: impl AsRef<[O]>) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_cargo-no-dev-deps"));
    cmd.current_dir(env!("CARGO_MANIFEST_DIR"));
    cmd.arg("no-dev-deps");
    cmd.args(args.as_ref());
    cmd
}

#[test]
fn help() {
    let short = cargo_no_dev_deps(["-h"]).assert_success();
    let long = cargo_no_dev_deps(["--help"]).assert_success();
    assert_eq!(short.stdout, long.stdout);
}

#[test]
fn version() {
    let expected = &format!("cargo-no-dev-deps {}", env!("CARGO_PKG_VERSION"));
    cargo_no_dev_deps(["-V"]).assert_success().stdout_eq(expected);
    cargo_no_dev_deps(["--version"]).assert_success().stdout_eq(expected);
}

#[test]
fn update_readme() {
    let new = cargo_no_dev_deps(["--help"]).assert_success().stdout;
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("README.md");
    let command = "cargo no-dev-deps --help";
    test_helper::doc::sync_command_output_to_markdown(path, "readme-long-help", command, new);
}
