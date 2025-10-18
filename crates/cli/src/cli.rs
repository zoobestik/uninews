use crate::command::Commands;
use crate::configure::configure;
use clap::Parser;
use clap::builder::styling;
use clap::builder::styling::{AnsiColor, Effects};
use uninews_collect::cli::run_collect;
use uninews_manage::cli::run_manage;

const STYLES: styling::Styles = styling::Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    // .placeholder(AnsiColor::BrightBlack.on_default())
    .error(AnsiColor::Red.on_default().effects(Effects::BOLD))
    .valid(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .invalid(AnsiColor::Yellow.on_default().effects(Effects::BOLD));

#[derive(Parser)]
#[command(
    version,
    styles = STYLES,
    about = "UniNews - Command-line interface for managing self-hosted web content collection and aggregation"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

pub async fn run() {
    configure();

    match Cli::parse().command {
        Commands::Collect(cmd) => run_collect(cmd).await,
        Commands::Manage(cmd) => run_manage(cmd).await,
    }
}
