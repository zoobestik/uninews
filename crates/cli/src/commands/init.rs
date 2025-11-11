use crate::cli::report::Report;
use crate::report::{ReportExt, ReportStatus};
use anyhow::{Context, Result};
use clap::Parser;
use news_sqlite_core::utils::fs::{create_parent_dirs, get_db_path, to_db_uri};
use sqlx::{SqlitePool, migrate};
use std::path::Path;
use tokio::fs::{remove_file, try_exists};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, stdin, stdout};

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

async fn confirm_overwrite(indent: &str) -> Result<bool> {
    let mut stdout = stdout();

    stdout
        .write_all(
            format!(
                "{}Database file already exists. Do you want to overwrite it? [y/N] ",
                indent
            )
            .as_bytes(),
        )
        .await
        .context("Failed to write prompt")?;

    stdout.flush().await.context("Failed to flush stdout")?;

    let mut reader = BufReader::new(stdin());
    let mut input = String::new();

    reader
        .read_line(&mut input)
        .await
        .context("Failed to read user input")?;

    Ok(input.trim().eq_ignore_ascii_case("y"))
}

async fn remove_database_if_exists(db_path: &Path, should_remove: bool) -> Result<()> {
    if should_remove {
        remove_file(db_path)
            .await
            .context("Failed to remove existing database file")?;
    }
    Ok(())
}

async fn create_database_directories(db_path: &Path) -> Result<()> {
    create_parent_dirs(db_path)
        .await
        .context("Failed to create parent directories")?;
    Ok(())
}

async fn connect_and_migrate(db_path: &Path, task_migrate: &Report) -> Result<()> {
    let db = task_migrate
        .simple("Connecting database", |_| {
            let db_uri = to_db_uri(db_path);
            Box::pin(async move {
                SqlitePool::connect(&db_uri)
                    .await
                    .context("Failed to connect to database")
            })
        })
        .await?;

    task_migrate
        .simple("Applying database migrations", |_| {
            Box::pin(async move {
                migrate!("../../migrations")
                    .run(&db)
                    .await
                    .context("Failed to run database migrations")
            })
        })
        .await?;

    Ok(())
}

pub async fn init_app(args: InitCommand) -> Result<()> {
    Report::task("Initializing database", |task_main| {
        Box::pin(async move {
            let db_path = get_db_path();

            let db_file_exists = try_exists(&db_path)
                .await
                .context("Failed to check if database file exists")?;

            if db_file_exists && !args.force && !confirm_overwrite(&task_main.indent_str()).await? {
                task_main.skipped();
                return Ok(());
            }

            task_main
                .simple("Removing existing database file", |task_remove| {
                    let db_path = db_path.clone();
                    Box::pin(async move {
                        if db_file_exists {
                            remove_database_if_exists(&db_path, true).await
                        } else {
                            task_remove.skipped();
                            Ok(())
                        }
                    })
                })
                .await?;

            task_main
                .simple("Create parent directories", |_| {
                    let db_path = db_path.clone();
                    Box::pin(async move { create_database_directories(&db_path).await })
                })
                .await?;

            task_main
                .complex("Running database migrations", |task_migrate| {
                    let db_path = db_path.clone();
                    Box::pin(async move { connect_and_migrate(&db_path, task_migrate).await })
                })
                .await?;

            Ok(())
        })
    })
    .await
}
