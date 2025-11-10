mod atom;
mod telegram;

use self::atom::{AddAtom, add_atom_source};
use self::telegram::{AddTelegram, add_telegram_source};
use anyhow::Result;
use clap::{Parser, Subcommand};
use news_core::repos::source::SourceRepository;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(about = "Add a new information source (such as Atom feed or Telegram channel)")]
pub struct AddCommand {
    #[command(subcommand)]
    command: AddCommands,
}

#[derive(Subcommand, Debug)]
pub enum AddCommands {
    #[command(about = "Add new Atom/RSS feed source", visible_aliases=["rss"])]
    Atom(AddAtom),

    #[command(about = "Add new Telegram channel source", visible_aliases=["tg"])]
    Telegram(AddTelegram),
}

pub async fn add_source(sources: Arc<impl SourceRepository>, command: AddCommand) -> Result<()> {
    match command.command {
        AddCommands::Atom(args) => add_atom_source(sources, args).await,
        AddCommands::Telegram(args) => add_telegram_source(sources, args).await,
    }
}
