use crate::command::Commands;
use crate::configure::configure;
use clap::Parser;
use uninews_collect::cli::run_collect;
use uninews_manage::cli::run_manage;

#[derive(Parser)]
#[command()]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

pub async fn run() {
    configure();

    match Cli::parse().command {
        Commands::Collect => run_collect().await,
        Commands::Manage(cmd) => run_manage(cmd).await,
    }
}
