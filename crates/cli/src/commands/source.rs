mod add;
mod list;
mod remove;

use self::add::{AddCommand, add_source};
use self::remove::{RemoveCommand, remove_source};
use crate::commands::source::list::{ArgsList, list_sources};
use clap::{Parser, Subcommand};
use sqlx::SqlitePool;
use std::error::Error;
use std::sync::Arc;
use uninews_core::fs::get_db_uri;
use uninews_core::repo::source::sqlite::SqliteSourceRepository;

#[derive(Parser, Debug)]
#[command(
    about = "Manage information sources like Atom feeds and Telegram channels",
    visible_aliases = ["src"],
)]
pub struct SourceCommand {
    #[command(subcommand)]
    command: SourceCommands,
}

#[derive(Debug, Subcommand)]
pub enum SourceCommands {
    Add(AddCommand),
    Remove(RemoveCommand),
    #[command(about = "List configured information sources", visible_aliases = ["ls"])]
    List(ArgsList),
}

pub async fn run_source(cmd: SourceCommand) -> Result<(), Box<dyn Error>> {
    let db_pool = SqlitePool::connect(&get_db_uri()?).await?;
    let source_repo = Arc::new(SqliteSourceRepository::new(db_pool));

    match cmd.command {
        SourceCommands::Add(cmd) => add_source(source_repo, cmd).await,
        SourceCommands::Remove(cmd) => remove_source(source_repo, cmd).await,
        SourceCommands::List(args) => list_sources(source_repo, args).await,
    }
}
