//! Developer tasks: `cargo xtask verify`.

#![allow(clippy::print_stderr)]

use std::path::Path;
use std::process::Command;

use anyhow::{Result, bail};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(about = "Qualia developer tasks")]
struct Cli {
    #[command(subcommand)]
    task: Task,
}

#[derive(Debug, Subcommand)]
enum Task {
    /// Formatting, Clippy at -D warnings, and tests.
    Verify,
}

fn main() -> Result<()> {
    match Cli::parse().task {
        Task::Verify => verify(),
    }
}

fn verify() -> Result<()> {
    cargo(&["fmt", "--all", "--", "--check"])?;
    cargo(&[
        "clippy",
        "--workspace",
        "--all-targets",
        "--locked",
        "--",
        "-D",
        "warnings",
    ])?;
    cargo(&["test", "--workspace", "--locked"])?;
    eprintln!("verify: ok");
    Ok(())
}

fn cargo(args: &[&str]) -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    eprintln!("$ cargo {}", args.join(" "));
    let status = Command::new(env!("CARGO"))
        .args(args)
        .current_dir(root)
        .status()?;
    if !status.success() {
        bail!("cargo {} failed with {status}", args.join(" "));
    }
    Ok(())
}
