use clap::Parser;
use sqlx::{migrate, sqlite};
use std::error::Error;
use std::io::Write;
use std::path::Path;
use std::{env, io};
use tokio::fs;
use tracing::info;
use uninews_core::fs::create_parent_dirs;

#[derive(Parser, Debug)]
#[command(about = "Initialize application database and create required directories")]
pub struct InitCommand {
    #[clap(
        short,
        long,
        help = "Override existing configuration without confirmation"
    )]
    force: bool,
}

pub async fn init_app(args: InitCommand) -> Result<(), Box<dyn Error>> {
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
