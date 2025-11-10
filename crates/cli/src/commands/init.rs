use crate::cli::report::Report;
use crate::report::Report as ReportTrait;

use anyhow::{Context, Result};
use clap::Parser;
use fs::{remove_file, try_exists};
use io::{stdin, stdout};
use news_sqlite_core::utils::fs::{create_parent_dirs, get_db_path, to_db_uri};
use sqlx::{SqlitePool, migrate};
use std::io;
use std::io::Write;
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
    let task_main = Report::new("Initializing database");

    let db_path = get_db_path();
    let db_file = db_path.as_path();

    let db_file_exists = try_exists(db_file)
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
            return Ok(());
        }
    }

    {
        let task_remove = task_main.simple("Removing existing database file");
        if db_file_exists {
            remove_file(db_file)
                .await
                .context("Failed to remove existing database file")?;
            task_remove.finish();
        } else {
            task_remove.skipped();
        }
    }

    {
        let task_dirs = task_main.simple("Create parent directories");
        create_parent_dirs(db_file)
            .await
            .context("Failed to create parent directories")?;
        task_dirs.finish();
    }

    {
        let task_migrate = task_main.complex("Running database migrations");

        let task_sub = task_migrate.simple("Connecting database");
        let db_uri = to_db_uri(db_file);
        let db = SqlitePool::connect(&db_uri)
            .await
            .context("Failed to connect to database")?;
        task_sub.finish();

        let task_sub = task_migrate.simple("Applying database migrations");
        migrate!("../../migrations")
            .run(&db)
            .await
            .context("Failed to run database migrations")?;
        task_sub.finish();

        task_migrate.finish();
    }

    task_main.finish();
    Ok(())
}
