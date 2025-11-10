use crate::commands::{Commands, run_commands};
use clap::Parser;
use clap::builder::styling;
use clap::builder::styling::AnsiColor::{Cyan, Green, Red, Yellow};
use clap::builder::styling::Effects;
use std::process::exit;
use styling::Styles;

const STYLES: Styles = Styles::styled()
    .header(Green.on_default().effects(Effects::BOLD))
    .usage(Green.on_default().effects(Effects::BOLD))
    .literal(Cyan.on_default().effects(Effects::BOLD))
    // .placeholder(BrightBlack.on_default())
    .error(Red.on_default().effects(Effects::BOLD))
    .valid(Cyan.on_default().effects(Effects::BOLD))
    .invalid(Yellow.on_default().effects(Effects::BOLD));

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
    if let Err(e) = run_commands(Cli::parse().command) {
        eprintln!("Error: {:#}", e);
        exit(1);
    }
}
