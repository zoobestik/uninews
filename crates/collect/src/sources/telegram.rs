use crate::sources::SourceOrigin;
use serde::{Deserialize, Deserializer};
use url::Url;

#[derive(Deserialize)]
struct TelegramChannelRaw {
    nickname: String,
}

impl TelegramChannelRaw {
    fn validate_nickname(name: &str) -> Result<(), String> {
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

        Ok(())
    }

    fn validate(self) -> Result<Self, String> {
        Self::validate_nickname(&self.nickname)?;
        Ok(self)
    }
}

#[derive(Debug)]
pub struct TelegramChannel {
    // name: String,
    url: Url,
}

impl<'de> Deserialize<'de> for TelegramChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let channel = TelegramChannelRaw::deserialize(deserializer)?
            .validate()
            .map_err(serde::de::Error::custom)?;

        let url = Url::parse(&format!("https://t.me/{}", channel.nickname))
            .map_err(serde::de::Error::custom)?;

        Ok(Self {
            url,
            /*name: channel.name*/
        })
    }
}

impl SourceOrigin for TelegramChannel {
    fn original_url(&self) -> String {
        self.url.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tg_from_nick(nick: &str) -> Result<TelegramChannel, toml::de::Error> {
        let nick = format!("nickname = \"{nick}\"");
        toml::from_str::<TelegramChannel>(&nick)
    }

    #[test]
    fn nickname_validation_successful() {
        assert_eq!(
            "https://t.me/abcde",
            tg_from_nick("abcde").expect("len 5 nick ok").original_url()
        );

        assert_eq!(
            "https://t.me/abcdefghij",
            tg_from_nick("abcdefghij")
                .expect("len 10 nick ok")
                .original_url()
        );

        let nick32 = "a".repeat(32);
        assert_eq!(
            format!("https://t.me/{nick32}"),
            tg_from_nick(&nick32)
                .expect("len 32 nick ok")
                .original_url()
        );

        assert_eq!(
            "https://t.me/rust_lang_2024",
            tg_from_nick("rust_lang_2024")
                .expect("allowed chars ok")
                .original_url(),
        );
    }

    #[test]
    fn nickname_validation_failures() {
        assert!(
            tg_from_nick("abcd")
                .unwrap_err()
                .to_string()
                .contains("too short")
        );

        assert!(
            tg_from_nick(&"a".repeat(33))
                .unwrap_err()
                .to_string()
                .contains("too long")
        );

        for example in ["abc de", "abc-de", "abc!d", "тесты", "naïve"] {
            let e = tg_from_nick(example).unwrap_err().to_string();
            assert!(e.contains("invalid nickname"), "message for {example}: {e}");
        }
    }

    #[test]
    fn deserialize_builds_correct_url() {
        assert_eq!(
            "https://t.me/rustaceans",
            tg_from_nick("rustaceans").unwrap().original_url(),
        );
    }
}
