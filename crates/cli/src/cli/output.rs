use console::colors_enabled;
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
