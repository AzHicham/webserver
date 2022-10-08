use crate::settings::Settings;
use rocket::futures::SinkExt;
use std::io;
use tracing::instrument::WithSubscriber;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// Create a subscriber to collect all logs
// that are created **in all threads**.
// Warning : this function will panic if called twice in the same program
// https://docs.rs/tracing/latest/tracing/dispatcher/index.html
pub fn init_logger(settings: &Settings) {
    let default_level = LevelFilter::INFO;
    let rust_log =
        std::env::var(EnvFilter::DEFAULT_ENV).unwrap_or_else(|_| default_level.to_string());
    let env_filter_subscriber = EnvFilter::try_new(rust_log).unwrap_or_else(|err| {
        eprintln!(
            "invalid {}, falling back to level '{}' - {}",
            EnvFilter::DEFAULT_ENV,
            default_level,
            err,
        );
        EnvFilter::new(default_level.to_string())
    });

    let file_appender = tracing_appender::rolling::daily("/Users/hazimani/dev", "webserver.log");
    let (non_blocking, _) = tracing_appender::non_blocking(file_appender);

    let collector = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_writer(io::stdout))
        .with(env_filter_subscriber)
        .with(fmt::Layer::new().with_writer(non_blocking));
    tracing::subscriber::set_global_default(collector)
        .expect("Failed to set global tracing subscriber.");

    tracing::info!("preparing to shave yaks");
}
