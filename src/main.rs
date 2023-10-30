// SPDX-License-Identifier: Apache-2.0 OR MIT

#![forbid(unsafe_code)]
#![allow(clippy::struct_field_names)]

#[macro_use]
mod term;

#[macro_use]
mod process;

mod cargo;
mod cli;
mod fs;
mod manifest;
mod metadata;
mod restore;

use std::env;

use anyhow::Result;

use crate::{cargo::Workspace, cli::Args};

fn main() {
    term::init_coloring();
    if let Err(e) = try_main() {
        error!("{e:#}");
    }
    if term::error()
        || term::warn()
            && env::var_os("CARGO_NO_DEV_DEPS_DENY_WARNINGS").filter(|v| v == "true").is_some()
    {
        std::process::exit(1)
    }
}

fn try_main() -> Result<()> {
    let args = Args::parse()?;
    let ws = Workspace::new(args.manifest_path.as_deref())?;

    let no_dev_deps = true;
    // TODO: provide option to keep updated Cargo.lock
    let restore_lockfile = true;
    manifest::with(&ws.metadata, no_dev_deps, args.no_private, restore_lockfile, || {
        let mut cargo = ws.cargo();
        cargo.args(args.cargo_args);
        if !args.rest.is_empty() {
            cargo.arg("--");
            cargo.args(args.rest);
        }
        info!("running {cargo}");
        cargo.run()
    })
}
