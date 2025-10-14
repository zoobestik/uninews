use tracing::dispatcher::SetGlobalDefaultError;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

/// Initialize tracing subscriber with environment filters.
///
/// This function sets up a global tracing subscriber with thread names, no target, and environment-based filtering.
/// The default log level is "info" if no environment filter is specified.
///
/// # Errors
///
/// Returns [`SetGlobalDefaultError`] if setting the global default subscriber fails, which can happen
/// if a global default subscriber was already set.
pub fn init_logger() -> Result<(), SetGlobalDefaultError> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let subscriber = FmtSubscriber::builder()
        .with_thread_names(true)
        .with_target(false)
        .with_env_filter(filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
}
