mod add;
mod remove;

use self::add::{AddCommand, add_source};
use self::remove::{RmArgs, remove_source};
use clap::{Parser, Subcommand};
use sqlx::SqlitePool;
use std::error::Error;
use std::sync::Arc;
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
    #[command(
        about = "Remove an information source (such as Atom feed or Telegram channel)",
        visible_aliases = ["rm"]
    )]
    Remove(RmArgs),
}

pub async fn run_source(cmd: SourceCommand) -> Result<(), Box<dyn Error>> {
    let db_pool = SqlitePool::connect("sqlite:./data/app.sqlite").await?;
    let source_repo = Arc::new(SqliteSourceRepository::new(db_pool));

    match cmd.command {
        SourceCommands::Add(cmd) => add_source(source_repo, cmd).await,
        SourceCommands::Remove(args) => remove_source(source_repo, args).await,
    }
}
