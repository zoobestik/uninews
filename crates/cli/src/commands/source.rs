mod add;
mod list;
mod remove;

use self::add::{AddCommand, add_source};
use self::list::{ArgsList, list_sources};
use self::remove::{RemoveCommand, remove_source};
use clap::{Parser, Subcommand};
use sqlx::SqlitePool;
use std::error::Error;
use std::sync::Arc;
use uninews_services::repos::source::SqliteSourceRepository;
use uninews_services::utils::fs::get_db_uri;

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

pub async fn run_source(cmd: SourceCommand) -> Result<(), Box<dyn Error>> {
    let db_pool = SqlitePool::connect(&get_db_uri()?).await?;
    let source_service = Arc::new(SqliteSourceRepository::new(db_pool));

    match cmd.command {
        SourceCommands::Add(cmd) => add_source(source_service, cmd).await,
        SourceCommands::Remove(cmd) => remove_source(source_service, cmd).await,
        SourceCommands::List(args) => list_sources(source_service, args).await,
    }
}
