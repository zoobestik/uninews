mod atom;
mod telegram;

use self::atom::{AddAtom, add_atom_source};
use self::telegram::{AddTelegramChannel, add_telegram_channel_source};
use clap::{Parser, Subcommand};
use std::error::Error;
use std::sync::Arc;
use uninews_core::repo::source::SourceRepository;

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
    Telegram(AddTelegramChannel),
}

pub async fn add_source(
    repo: Arc<impl SourceRepository>,
    command: AddCommand,
) -> Result<(), Box<dyn Error>> {
    match command.command {
        AddCommands::Atom(args) => add_atom_source(repo, args).await,
        AddCommands::Telegram(args) => add_telegram_channel_source(repo, args).await,
    }
}
