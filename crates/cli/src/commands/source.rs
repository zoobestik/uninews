mod add;
mod list;
mod remove;

use self::add::{AddCommand, add_source};
use self::list::{ArgsList, list_sources};
use self::remove::{RemoveCommand, remove_source};
use anyhow::Result;
use clap::{Parser, Subcommand};
use news_sqlite_core::repos::source::SqliteSourceRepository;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(
    about = "Manage information sources (such as Atom feed or Telegram channel)",
    visible_aliases = ["src"],
)]
pub struct SourceCommand {
    #[command(subcommand)]
    command: SourceCommands,
}

#[derive(Debug, Subcommand)]
pub enum SourceCommands {
    #[command(about = "List configured information sources", visible_aliases = ["ls"])]
    List(ArgsList),
    Add(AddCommand),
    Remove(RemoveCommand),
}

pub async fn run_source(cmd: SourceCommand) -> Result<()> {
    let source_service = Arc::new(SqliteSourceRepository::new().await?);

    match cmd.command {
        SourceCommands::Add(cmd) => add_source(source_service, cmd).await,
        SourceCommands::Remove(cmd) => remove_source(source_service, cmd).await,
        SourceCommands::List(args) => list_sources(source_service, args).await,
    }
}
