mod errors;
pub mod output;
pub mod report;

use self::output::OutputConfig;
use crate::commands::{Commands, run_commands};
use clap::Parser;
use clap::builder::styling::AnsiColor::{Cyan, Green, Red, Yellow};
use clap::builder::styling::Effects;
use clap::builder::styling::Styles;
use console::{colors_enabled, set_colors_enabled};
use errors::display_error;
use std::process::exit;

const STYLES: Styles = Styles::styled()
    .header(Green.on_default().effects(Effects::BOLD))
    .usage(Green.on_default().effects(Effects::BOLD))
    .literal(Cyan.on_default().effects(Effects::BOLD))
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

    #[arg(short, long, global = true)]
    verbose: bool,

    #[arg(long, global = true)]
    no_color: bool,
}

pub async fn run() {
    let cli = Cli::parse();

    if cli.no_color {
        set_colors_enabled(false);
    }

    OutputConfig::init(cli.verbose, colors_enabled());

    if let Err(e) = run_commands(cli.command).await {
        display_error(&e);
        exit(1);
    }
}
