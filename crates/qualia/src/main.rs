//! `qualia check <world.json>`: validate a world graph and print its step order.

#![allow(clippy::print_stdout)]

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use qualia_core::WorldGraph;

#[derive(Debug, Parser)]
#[command(version, about = "Qualia world tools")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Validate a world graph and print its per-step operator order.
    Check { path: PathBuf },
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    match Cli::parse().command {
        Command::Check { path } => {
            let text = std::fs::read_to_string(&path)
                .with_context(|| format!("reading {}", path.display()))?;
            let graph: WorldGraph = serde_json::from_str(&text)
                .with_context(|| format!("parsing {}", path.display()))?;
            let schedule = graph
                .schedule()
                .with_context(|| format!("validating {}", path.display()))?;
            tracing::debug!(operators = schedule.order.len(), "world graph is valid");
            for id in schedule.order {
                println!("{id}");
            }
            Ok(())
        }
    }
}
