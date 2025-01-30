// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg(not(miri))] // Miri doesn't support file with non-default mode: https://github.com/rust-lang/miri/pull/2720

use std::{env, ffi::OsStr, path::Path, process::Command};

use fs_err as fs;
use test_helper::{cli::CommandExt as _, git::assert_diff};

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
    let new = &*cargo_no_dev_deps(["--help"]).assert_success().stdout;
    let path = &Path::new(env!("CARGO_MANIFEST_DIR")).join("README.md");
    let base = fs::read_to_string(path).unwrap();
    let mut out = String::with_capacity(base.capacity());
    let mut lines = base.lines();
    let mut start = false;
    let mut end = false;
    while let Some(line) = lines.next() {
        out.push_str(line);
        out.push('\n');
        if line == "<!-- readme-long-help:start -->" {
            start = true;
            out.push_str("```console\n");
            out.push_str("$ cargo no-dev-deps --help\n");
            out.push_str(new);
            for line in &mut lines {
                if line == "<!-- readme-long-help:end -->" {
                    out.push_str("```\n");
                    out.push_str(line);
                    out.push('\n');
                    end = true;
                    break;
                }
            }
        }
    }
    if start && end {
        assert_diff(path, out);
    } else if start {
        panic!("missing `<!-- readme-long-help:end -->` comment in README.md");
    } else {
        panic!("missing `<!-- readme-long-help:start -->` comment in README.md");
    }
}
