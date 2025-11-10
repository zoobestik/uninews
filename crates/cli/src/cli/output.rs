use console::{colors_enabled, style};
use dotenvy::var;
use std::sync::OnceLock;

pub struct OutputConfig {
    pub verbose: bool,
    pub colors_enabled: bool,
}

static OUTPUT_CONFIG: OnceLock<OutputConfig> = OnceLock::new();

impl OutputConfig {
    pub fn init(verbose: bool, colors_enabled: bool) {
        OUTPUT_CONFIG.get_or_init(|| Self {
            verbose,
            colors_enabled,
        });
    }

    pub fn get() -> &'static Self {
        OUTPUT_CONFIG.get_or_init(|| Self {
            verbose: var("RUST_LOG").is_ok() || var("DEBUG").is_ok(),
            colors_enabled: colors_enabled(),
        })
    }

    pub fn is_verbose() -> bool {
        Self::get().verbose
    }
}

pub fn success(msg: &str) {
    println!("{} {}", style("✓").green().bold(), msg);
}

pub fn info(msg: &str) {
    println!("{} {}", style("→").cyan(), msg);
}

pub fn running(msg: &str) {
    println!("{:>12} {}", style("Running").green().bold(), msg);
}

pub fn finished(msg: &str) {
    println!("{:>12} {}", style("Finished").green().bold(), msg);
}
