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
    match command {
        Commands::Collect(cmd) => {
            if let Err(e) = run_collect(cmd).await {
                eprintln!("Error initializing collecting: {e}");
                exit(1);
            }
        }
        Commands::Init(cmd) => {
            if let Err(e) = init_app(cmd).await {
                eprintln!("Error initializing database: {e}");
                exit(1);
            }
        }
        Commands::Source(cmd) => {
            if let Err(e) = run_source(cmd).await {
                eprintln!("Error in source command: {e}");
                exit(1);
            }
        }
    }
}
