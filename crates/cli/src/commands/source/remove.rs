mod atom;
mod telegram;

use self::atom::{RemoveAtom, remove_atom_source};
use self::telegram::{RemoveTelegram, remove_telegram_source};
use anyhow::Result;
use clap::{Parser, Subcommand};
use news_core::repos::source::SourceRepository;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(
    about = "Remove an information source (such as Atom feed or Telegram channel)",
    visible_aliases = ["rm"]
)]
pub struct RemoveCommand {
    #[command(subcommand)]
    command: RemoveCommands,
}

#[derive(Subcommand, Debug)]
pub enum RemoveCommands {
    #[command(about = "Remove an Atom/RSS feed source", visible_aliases=["rss"])]
    Atom(RemoveAtom),

    #[command(about = "Remove a Telegram channel source", visible_aliases=["tg"])]
    Telegram(RemoveTelegram),
}

pub async fn remove_source(
    sources: Arc<impl SourceRepository>,
    command: RemoveCommand,
) -> Result<()> {
    match command.command {
        RemoveCommands::Atom(args) => remove_atom_source(sources, args).await,
        RemoveCommands::Telegram(args) => remove_telegram_source(sources, args).await,
    }
}
