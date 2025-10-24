mod atom;
mod telegram;

use self::atom::{RemoveAtom, rm_atom_source};
use self::telegram::{RemoveTelegramChannel, rm_telegram_channel_source};
use clap::{Parser, Subcommand};
use std::error::Error;
use std::sync::Arc;
use uninews_core::repo::source::SourceRepository;

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
    Telegram(RemoveTelegramChannel),
}

pub async fn remove_source(
    repo: Arc<impl SourceRepository>,
    command: RemoveCommand,
) -> Result<(), Box<dyn Error>> {
    match command.command {
        RemoveCommands::Atom(args) => rm_atom_source(repo, args).await,
        RemoveCommands::Telegram(args) => rm_telegram_channel_source(repo, args).await,
    }
}
