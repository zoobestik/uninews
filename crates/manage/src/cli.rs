use clap::{Args, Parser, Subcommand};
use sqlx::{migrate, sqlite};
use std::env;
use std::io::{self, Write};
use std::path::Path;
use tokio::fs;
use tracing::info;
use uninews_core::fs::create_parent_dirs;

#[derive(Parser, Debug)]
#[command(
    about = "Administrative CLI tool for managing users, content feeds and system configuration",
    visible_alias = "adm"
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
    force: bool,
}

pub async fn run_manage(cmd: ManageCommand) {
    match cmd.command {
        ManageCommands::Init(args) => {
            if let Err(e) = init_database(args).await {
                eprintln!("Error initializing database: {}", e);
            }
        }
    };
}

async fn init_database(args: InitArgs) -> Result<(), Box<dyn std::error::Error>> {
    let db_path = env::var("UNINEWS_DB_PATH").unwrap_or("data/app.sqlite".to_string());
    let db_file = Path::new(&db_path);
    let db_file_exists = fs::try_exists(db_file).await.unwrap_or(false);

    if db_file_exists && !args.force {
        info!("Database file already exists. Do you want to override it? [y/N] ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if !input.trim().eq_ignore_ascii_case("y") {
            return Ok(());
        }
    }

    if db_file_exists {
        fs::remove_file(db_file)
            .await
            .map_err(|e| format!("Failed to remove existing database: {e}"))?;
    }

    create_parent_dirs(db_file).await?;

    let db_uri = format!("sqlite:{0}?mode=rwc", db_file.display());
    let db = sqlite::SqlitePool::connect(&db_uri).await?;

    migrate!("../../migrations")
        .run(&db)
        .await
        .map_err(|e| format!("Failed to run database migrations: {e}"))?;

    info!("Database initialized successfully");
    Ok(())
}
