use clap::{Parser, Subcommand};
use tracing::debug;

#[derive(Parser, Debug)]
pub struct ManageCommand {
    #[command(subcommand)]
    command: ManageCommands,
}

#[derive(Subcommand, Debug)]
pub enum ManageCommands {
    Init,
}

pub async fn run_manage(cmd: ManageCommand) {
    match cmd.command {
        ManageCommands::Init => debug!("[run_manage]: <Init> done"),
    };
}
