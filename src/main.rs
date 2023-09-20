// Main entrypoint.

// Directives.
#![warn(rustdoc::broken_intra_doc_links, rust_2018_idioms, clippy::all)]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

// Modules.

pub mod base;
pub mod services;

// Imports.

use base::{types::Void, config::Config};
use clap::{command, Parser, Subcommand};
use yansi::Paint;

// Commands.

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, default_value = ".hidden/.augre.toml")]
    config: String,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Performs a code review of the current `git diff HEAD^`.
    Review,
}

// Entrypoint.

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Err(err) = start(args).await {
        eprintln!("{}: {}", Paint::red("ERROR"), err);
        std::process::exit(1);
    }
}

async fn start(args: Args) -> Void {
    let config = base::config::Config::new(&args.config)?;

    match args.command {
        Some(Command::Review) => review(&config).await?,
        None => return Err(anyhow::anyhow!("No command specified.")),
    }

    Ok(())
}

async fn review(config: &Config) -> Void {

    Ok(())
}