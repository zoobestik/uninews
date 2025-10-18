use clap::Subcommand;
use uninews_manage::cli::ManageCommand;

#[derive(Subcommand)]
pub enum Commands {
    Collect,
    Manage(ManageCommand),
    // Config,
}
