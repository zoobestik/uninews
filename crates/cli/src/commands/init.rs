use anyhow::{Context, Result};
use clap::Parser;
use fs::{remove_file, try_exists};
use io::{stdin, stdout};
use news_sqlite_core::utils::fs::{create_parent_dirs, get_db_path, to_db_uri};
use sqlx::{SqlitePool, migrate};
use std::io;
use std::io::Write;
use tokio::fs;
use tracing::info;

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

pub async fn init_app(args: InitCommand) -> Result<()> {
    let db_path = get_db_path()?;
    let db_file = db_path.as_path();

    let db_file_exists = try_exists(db_file)
        .await
        .context("Failed to check if database file exists")?;

    if db_file_exists && !args.force {
        info!("Database file already exists. Do you want to overwrite it? [y/N] ");
        stdout().flush().context("Failed to flush stdout")?;

        let mut input = String::new();

        stdin()
            .read_line(&mut input)
            .context("Failed to read user input")?;

        if !input.trim().eq_ignore_ascii_case("y") {
            return Ok(());
        }
    }

    if db_file_exists {
        remove_file(db_file)
            .await
            .context("Failed to remove existing database file")?;
    }

    create_parent_dirs(db_file)
        .await
        .context("Failed to create parent directories")?;

    println!("Initializing database...");

    let db_uri = to_db_uri(db_file);
    let db = SqlitePool::connect(&db_uri)
        .await
        .context("Failed to connect to database")?;

    migrate!("../../migrations")
        .run(&db)
        .await
        .with_context(|e| format!("Failed to run database migrations: {e}"))?;

    println!("✓ Database initialized at: {}", db_path);
    println!("✓ All migrations applied successfully");
    Ok(())
}
