use clap::Subcommand;
use uninews_collect::cli::CollectCommand;
use uninews_manage::cli::ManageCommand;

#[derive(Subcommand)]
pub enum Commands {
    Collect(CollectCommand),
    Manage(ManageCommand),
    // Config,
}
