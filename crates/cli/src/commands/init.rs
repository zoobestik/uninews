use crate::cli::report::Report;
use crate::report::{ReportExt, ReportStatus};
use anyhow::{Context, Error, Result};
use clap::Parser;
use fs::{remove_file, try_exists};
use io::{stdin, stdout};
use news_sqlite_core::utils::fs::{create_parent_dirs, get_db_path, to_db_uri};
use sqlx::{SqlitePool, migrate};
use std::io;
use std::io::Write;
use std::sync::Arc;
use tokio::fs;

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
    Report::task("Initializing database", |task_main| {
        Box::pin(async move {
            let db_path = Arc::new(get_db_path());

            let db_file_exists = try_exists(&db_path.as_path())
                .await
                .context("Failed to check if database file exists")?;

            if db_file_exists && !args.force {
                println!(
                    "{}Database file already exists. Do you want to overwrite it? [y/N] ",
                    task_main.indent_str()
                );
                stdout().flush().context("Failed to flush stdout")?;

                let mut input = String::new();

                stdin()
                    .read_line(&mut input)
                    .context("Failed to read user input")?;

                if !input.trim().eq_ignore_ascii_case("y") {
                    task_main.skipped();
                    return Ok(());
                }
            }

            task_main
                .simple("Removing existing database file", |task_remove| {
                    let db_path = Arc::clone(&db_path);
                    Box::pin(async move {
                        match db_file_exists {
                            true => remove_file(&db_path.as_path())
                                .await
                                .context("Failed to remove existing database file"),
                            false => {
                                task_remove.skipped();
                                Ok::<(), Error>(())
                            }
                        }
                    })
                })
                .await?;

            task_main
                .simple("Create parent directories", |_| {
                    let db_path = Arc::clone(&db_path);
                    Box::pin(async move {
                        create_parent_dirs(db_path.as_path())
                            .await
                            .context("Failed to create parent directories")?;
                        Ok::<(), Error>(())
                    })
                })
                .await?;

            task_main
                .complex("Running database migrations", |task_migrate| {
                    let db_path = Arc::clone(&db_path);
                    Box::pin(async move {
                        let db = task_migrate
                            .simple("Connecting database", |_| {
                                let db_path = Arc::clone(&db_path);
                                Box::pin(async move {
                                    let db_uri = to_db_uri(db_path.as_path());
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

                        Ok::<(), Error>(())
                    })
                })
                .await?;

            Ok(())
        })
    })
    .await
}
