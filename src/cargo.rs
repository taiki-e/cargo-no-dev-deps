use std::{env, ffi::OsStr, path::PathBuf};

use anyhow::{Context as _, Result};
use serde::Deserialize;

use crate::process::ProcessBuilder;

pub(crate) struct Workspace {
    pub(crate) metadata: cargo_metadata::Metadata,
    cargo: PathBuf,
}

impl Workspace {
    pub(crate) fn new(manifest_path: Option<&str>) -> Result<Self> {
        let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());

        // Metadata
        let current_manifest_path = package_root(&cargo, manifest_path)?;
        let metadata = metadata(&cargo, &current_manifest_path)?;

        Ok(Self { cargo: cargo.into(), metadata })
    }

    pub(crate) fn cargo(&self) -> ProcessBuilder {
        cmd!(&self.cargo)
    }
}

fn package_root(cargo: &OsStr, manifest_path: Option<&str>) -> Result<String> {
    let package_root = if let Some(manifest_path) = manifest_path {
        manifest_path.to_owned()
    } else {
        locate_project(cargo)?
    };
    Ok(package_root)
}

// Old cargo doesn't support `--message-format plain`.
#[derive(Deserialize)]
struct LocateProject {
    root: String,
}

// https://doc.rust-lang.org/nightly/cargo/commands/cargo-locate-project.html
fn locate_project(cargo: &OsStr) -> Result<String> {
    Ok(serde_json::from_str::<LocateProject>(&cmd!(cargo, "locate-project").read()?)?.root)
}

// https://doc.rust-lang.org/nightly/cargo/commands/cargo-metadata.html
fn metadata(cargo: &OsStr, manifest_path: &str) -> Result<cargo_metadata::Metadata> {
    let mut cmd = cmd!(
        cargo,
        "metadata",
        "--format-version=1",
        "--no-deps",
        "--manifest-path",
        manifest_path
    );
    let json = cmd.read()?;
    serde_json::from_str(&json).with_context(|| format!("failed to parse output from {cmd}"))
}
