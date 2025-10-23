mod init;
mod source;

use self::init::{InitCommand, init_app};
use self::source::{SourceCommand, run_source};
use clap::Subcommand;
use std::process::exit;
use uninews_collect::cli::{CollectCommand, run_collect};

#[derive(Subcommand)]
pub enum Commands {
    Collect(CollectCommand),
    Init(InitCommand),
    Source(SourceCommand),
}

pub async fn run_commands(command: Commands) {
    let _ = match command {
        Commands::Collect(cmd) => run_collect(cmd)
            .await
            .map_err(|e| format!("Error in collect command: {e}")),

        Commands::Init(cmd) => init_app(cmd)
            .await
            .map_err(|e| format!("Error initializing database: {e}")),

        Commands::Source(cmd) => run_source(cmd)
            .await
            .map_err(|e| format!("Error in source command: {e}")),
    }
    .map_err(|e| {
        eprintln!("{}", e);
        exit(1);
    });
}
