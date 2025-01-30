// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{env, ffi::OsString};

use anyhow::{bail, format_err, Result};
use lexopt::{
    Arg::{Long, Short, Value},
    ValueExt as _,
};

use crate::term;

static USAGE: &str = "cargo-no-dev-deps\n
Cargo subcommand for running cargo without dev-dependencies.
\nUSAGE:
    cargo no-dev-deps <CARGO_SUBCOMMAND> [OPTIONS] [CARGO_OPTIONS]
\nOPTIONS:
        --no-private    Perform without `publish = false` crates
\nCARGO_SUBCOMMANDS:
    build
    check
    ...
";

pub(crate) struct Args {
    pub(crate) no_private: bool,
    pub(crate) manifest_path: Option<String>,
    pub(crate) cargo_args: Vec<String>,
    pub(crate) rest: Vec<String>,
}

impl Args {
    pub(crate) fn parse() -> Result<Option<Self>> {
        const SUBCMD: &str = "no-dev-deps";

        // rustc/cargo args must be valid Unicode
        // https://github.com/rust-lang/rust/blob/1.84.0/compiler/rustc_driver_impl/src/args.rs#L121
        // TODO: https://github.com/rust-lang/cargo/pull/11118
        fn handle_args(
            args: impl IntoIterator<Item = impl Into<OsString>>,
        ) -> impl Iterator<Item = Result<String>> {
            args.into_iter().enumerate().map(|(i, arg)| {
                arg.into()
                    .into_string()
                    .map_err(|arg| format_err!("argument {} is not valid Unicode: {arg:?}", i + 1))
            })
        }

        let mut raw_args = handle_args(env::args_os());
        raw_args.next(); // cargo
        match raw_args.next().transpose()? {
            Some(a) if a == SUBCMD => {}
            Some(a) => bail!("expected subcommand '{SUBCMD}', found argument '{a}'"),
            None => bail!("expected subcommand '{SUBCMD}'"),
        }
        let mut args = vec![];
        for arg in &mut raw_args {
            let arg = arg?;
            if arg == "--" {
                break;
            }
            args.push(arg);
        }
        let rest = raw_args.collect::<Result<Vec<_>>>()?;

        let mut cargo_args = vec![];
        let mut subcommand = None;
        let mut color = None;
        let mut manifest_path: Option<String> = None;
        let mut verbose = 0;

        let mut no_private = false;

        let mut parser = lexopt::Parser::from_args(args);
        while let Some(arg) = parser.next()? {
            macro_rules! parse_opt {
                ($opt:ident $(,)?) => {{
                    if $opt.is_some() {
                        multi_arg(&arg)?;
                    }
                    $opt = Some(parser.value()?.parse()?);
                }};
            }
            macro_rules! parse_flag {
                ($flag:ident $(,)?) => {{
                    if $flag {
                        multi_arg(&arg)?;
                    }
                    $flag = true;
                }};
            }

            match arg {
                Long("color") => parse_opt!(color),
                Long("manifest-path") => parse_opt!(manifest_path),
                Short('v') | Long("verbose") => verbose += 1,

                Long("no-private") => parse_flag!(no_private),

                Short('h') | Long("help") if subcommand.is_none() => {
                    print!("{USAGE}");
                    return Ok(None);
                }
                Short('V') | Long("version") if subcommand.is_none() => {
                    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                    return Ok(None);
                }

                // passthrough
                Long(flag) => {
                    let flag = format!("--{flag}");
                    if let Some(val) = parser.optional_value() {
                        cargo_args.push(format!("{flag}={}", val.string()?));
                    } else {
                        cargo_args.push(flag);
                    }
                }
                Short(flag) => {
                    if matches!(flag, 'n' | 'q' | 'r') {
                        // To handle combined short flags properly, handle known
                        // short flags without value as special cases.
                        cargo_args.push(format!("-{flag}"));
                    } else if let Some(val) = parser.optional_value() {
                        cargo_args.push(format!("-{flag}{}", val.string()?));
                    } else {
                        cargo_args.push(format!("-{flag}"));
                    }
                }
                Value(val) => {
                    let val = val.string()?;
                    if subcommand.is_none() {
                        subcommand = Some(());
                    }
                    cargo_args.push(val);
                }
            }
        }

        term::set_coloring(color)?;

        term::verbose::set(verbose != 0);
        // If `-vv` is passed, propagate `-v` to cargo.
        if verbose > 1 {
            cargo_args.push(format!("-{}", "v".repeat(verbose - 1)));
        }
        if let Some(color) = color {
            cargo_args.push("--color".to_owned());
            cargo_args.push(color.as_str().to_owned());
        }
        if let Some(path) = &manifest_path {
            cargo_args.push("--manifest-path".to_owned());
            cargo_args.push(path.clone());
        }

        Ok(Some(Self { no_private, manifest_path, cargo_args, rest }))
    }
}

fn format_flag(flag: &lexopt::Arg<'_>) -> String {
    match flag {
        Long(flag) => format!("--{flag}"),
        Short(flag) => format!("-{flag}"),
        Value(_) => unreachable!(),
    }
}

#[cold]
#[inline(never)]
fn multi_arg(flag: &lexopt::Arg<'_>) -> Result<()> {
    let flag = &format_flag(flag);
    bail!("argument '{flag}' was provided more than once, but cannot be used multiple times")
}
