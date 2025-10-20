mod add;

use self::add::{AddArgs, add_source};
use clap::{Parser, Subcommand};
use sqlx::SqlitePool;
use std::error::Error;
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
    #[clap(about = "Add a new information source (such as Atom feed or Telegram channel)")]
    Add(AddArgs),
}

pub async fn run_source(cmd: SourceCommand) -> Result<(), Box<dyn Error>> {
    let db_pool = SqlitePool::connect("sqlite:./data/app.sqlite").await?;
    let source_repo = SqliteSourceRepository::new(db_pool);
    match cmd.command {
        SourceCommands::Add(args) => add_source(source_repo, args).await,
    }
}
