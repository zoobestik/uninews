use clap::{Args, Parser, Subcommand};
use tracing::debug;

#[derive(Parser, Debug)]
#[command(
    about = "Administrative CLI tool for managing users, content feeds and system configuration",
    visible_alias = "man"
)]
pub struct ManageCommand {
    #[command(subcommand)]
    command: ManageCommands,
}

#[derive(Subcommand, Debug)]
pub enum ManageCommands {
    #[clap(about = "Initialize a new application instance")]
    Init(InitArgs),
}

#[derive(Args, Debug)]
pub struct InitArgs {
    #[clap(
        short,
        long,
        help = "Override existing configuration without confirmation"
    )]
    force: bool, // @todo: when true, skip confirmation prompts and override existing configuration without asking
}

pub async fn run_manage(cmd: ManageCommand) {
    match cmd.command {
        ManageCommands::Init(args) => debug!("[run_manage]: <Init> done: {args:?}"),
    };
}
