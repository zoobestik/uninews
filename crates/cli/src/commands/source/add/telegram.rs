use crate::commands::source::SourceService;
use clap::Args;
use std::error::Error;
use uninews_core::models::telegram::TelegramChannelDraft;
use uninews_core::repo::source::SourceCreate;

#[derive(Debug, Args)]
pub struct AddTelegramChannel {
    #[arg(value_parser = parse_telegram_username)]
    username: String,
}

fn parse_telegram_username(name: &str) -> Result<String, String> {
    if name.len() > 32 {
        return Err(format!(
            "{name} is too long for a nickname; it must be less than 32 characters."
        ));
    }

    if name.len() < 5 {
        return Err(format!(
            "{name} is too short for a nickname; it must be at least 5 characters."
        ));
    }

    let has_invalid_chars = name
        .chars()
        .any(|c| !(c.is_ascii_alphanumeric() || c == '_'));

    if has_invalid_chars {
        return Err(format!(
            "{name} is an invalid nickname. It must contain only letters, numbers, and underscores."
        ));
    }

    Ok(name.to_string())
}

pub async fn add_telegram_channel_source(
    repo: SourceService,
    args: AddTelegramChannel,
) -> Result<(), Box<dyn Error>> {
    repo.insert(SourceCreate::TelegramChannel(TelegramChannelDraft::new(
        args.username,
    )))
    .await?;
    Ok(())
}
