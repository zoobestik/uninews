mod collect;
mod init;
mod source;

use self::collect::{CollectCommand, run_collect};
use self::init::{InitCommand, init_app};
use self::source::{SourceCommand, run_source};
use crate::configure::configure;
use anyhow::{Context, Result};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Collect(CollectCommand),
    Init(InitCommand),
    Source(SourceCommand),
}

pub async fn run_commands(command: Commands) -> Result<()> {
    configure()?;

    match command {
        Commands::Collect(cmd) => run_collect(cmd).await.context("Collect command failed"),
        Commands::Init(cmd) => init_app(cmd).await.context("Initialization failed"),
        Commands::Source(cmd) => run_source(cmd).await.context("Source command failed"),
    }
}
