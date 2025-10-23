use super::SourceService;
use clap::{Args, Parser, Subcommand};
use std::error::Error;
use uninews_core::models::atom::AtomDraft;
use uninews_core::repo::source::SourceCreate;
use uninews_core::url::parse_url;
use url::Url;

#[derive(Parser, Debug)]
#[command(about = "Add a new information source (such as Atom feed or Telegram channel)")]
pub struct AddCommand {
    #[command(subcommand)]
    command: AddCommands,
}

#[derive(Subcommand, Debug)]
pub enum AddCommands {
    #[command(about = "Add new Atom/RSS feed source", visible_aliases=["rss"])]
    Atom(AddAtomArgs),
}

#[derive(Debug, Args)]
pub struct AddAtomArgs {
    #[arg(value_parser = parse_url)]
    url: Url,
}

pub async fn add_source(repo: SourceService, command: AddCommand) -> Result<(), Box<dyn Error>> {
    match command.command {
        AddCommands::Atom(args) => add_atom_source(repo, args).await,
    }
}

async fn add_atom_source(repo: SourceService, args: AddAtomArgs) -> Result<(), Box<dyn Error>> {
    repo.insert(SourceCreate::Atom(AtomDraft::new(args.url)))
        .await?;
    Ok(())
}
