// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{env, ffi::OsStr, path::PathBuf};

use anyhow::{format_err, Result};

use crate::{metadata, process::ProcessBuilder};

pub(crate) struct Workspace {
    pub(crate) metadata: metadata::Metadata,
    cargo: PathBuf,
}

impl Workspace {
    pub(crate) fn new(manifest_path: Option<&str>) -> Result<Self> {
        let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
        let cargo_version = cargo_version(&cargo)?;

        let metadata = metadata::Metadata::new(manifest_path, &cargo, cargo_version.minor)?;

        Ok(Self { cargo: cargo.into(), metadata })
    }

    pub(crate) fn cargo(&self) -> ProcessBuilder {
        cmd!(&self.cargo)
    }
}

fn cargo_version(cargo: &OsStr) -> Result<CargoVersion> {
    // Use verbose version output because the packagers add extra strings to the normal version output.
    let mut cmd = cmd!(cargo, "--version", "--verbose");
    let verbose_version = cmd.read()?;
    CargoVersion::parse(&verbose_version)
        .ok_or_else(|| format_err!("unexpected version output from {cmd}: {verbose_version}"))
}

struct CargoVersion {
    minor: u32,
}

impl CargoVersion {
    fn parse(verbose_version: &str) -> Option<Self> {
        let release = verbose_version.lines().find_map(|line| line.strip_prefix("release: "))?;
        let (version, _channel) = release.split_once('-').unwrap_or((release, ""));
        let mut digits = version.splitn(3, '.');
        let major = digits.next()?;
        if major != "1" {
            return None;
        }
        let minor = digits.next()?.parse::<u32>().ok()?;
        let _patch = digits.next()?.parse::<u32>().ok()?;

        Some(Self { minor })
    }
}
