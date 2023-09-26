// Main entrypoint.

// Directives.
#![warn(rustdoc::broken_intra_doc_links, rust_2018_idioms, clippy::all)]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
#![feature(fs_try_exists)]
#![feature(vec_into_raw_parts)]

// Modules.

pub mod base;
pub mod services;

// Imports.

use base::{types::{Void, EnsurableEntity, Mode, RemovableEntity}, config::Config};
use clap::{command, Parser, Subcommand};
use services::{git::Git, gpt::Gpt, cria::Cria};
use termimad::MadSkin;
use yansi::Paint;

use crate::services::{docker::Docker, model::Model};

// Commands.

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, default_value = ".augre")]
    data_path: String,

    #[arg(short, long, default_value = "openai")]
    mode: Mode,

    #[clap(long, short, action)]
    skip_confirm: bool,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Performs a code review of the current `git diff HEAD^`.
    Review,

    /// Gives a response to the specified prompt.
    Ask {
        /// The prompt to respond to.
        prompt: String,
    },

    /// Stop all of the background services.
    Stop,
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
    let config = base::config::Config::new(&args.data_path, args.mode)?;
    let confirm = !args.skip_confirm;

    match args.command {
        Some(Command::Review) => review(&config, confirm).await?,
        Some(Command::Ask { prompt }) => ask(&config, confirm, &prompt).await?,
        Some(Command::Stop) => stop(&config, confirm).await?,
        None => return Err(anyhow::anyhow!("No command specified.")),
    }

    Ok(())
}

async fn review(config: &Config, confirm: bool) -> Void {
    println!();

    maybe_prepare_local(config, confirm).await?;

    let git = Git::default();
    let gpt = Gpt::new(&config.openai_endpoint, &config.openai_key, config.mode);

    git.ensure(confirm).await?;
    gpt.ensure(confirm).await?;

    println!();

    print!("Getting diff ...");
    let diff = Git::diff().await?;
    println!(" {}", Paint::green("✔️"));

    println!("Getting review ...");
    let response = gpt.review(&diff).await?.trim().to_string();
    println!("{}", Paint::green("✔️"));

    println!();

    let skin = MadSkin::default();
    skin.print_text(&response);

    Ok(())
}

async fn ask(config: &Config, confirm: bool, prompt: &str) -> Void {
    println!();

    maybe_prepare_local(config, confirm).await?;

    let gpt = Gpt::new(&config.openai_endpoint, &config.openai_key, config.mode);
    gpt.ensure(confirm).await?;

    println!();

    println!("Getting response ...");
    let response = gpt.ask(prompt).await?.trim().to_string();
    println!("{}", Paint::green("✔️"));

    println!();

    let skin = MadSkin::default();
    skin.print_text(&response);

    Ok(())
}

async fn stop(config: &Config, confirm: bool) -> Void {
    let cria = Cria::new(&config.model_path, &config.data_path, config.mode, config.cria_port);

    cria.remove(confirm).await?;

    Ok(())
}

async fn maybe_prepare_local(config: &Config, confirm: bool) -> Void {
    if config.mode == Mode::LocalCpu || config.mode == Mode::LocalGpu {
        let docker = Docker::default();
        let model = Model::new(&config.model_path, &config.model_url);
        let cria = Cria::new(&config.model_path, &config.data_path, config.mode, config.cria_port);

        docker.ensure(confirm).await?;
        model.ensure(confirm).await?;
        cria.ensure(confirm).await?;
    }

    Ok(())
}