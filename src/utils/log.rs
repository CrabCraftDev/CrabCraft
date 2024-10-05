pub mod formatter;

use std::fs::OpenOptions;

use formatter::LogFormatter;
use supports_color::Stream;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt::writer::MakeWriterExt, layer::SubscriberExt, util::SubscriberInitExt};

use super::files::FILES;

pub fn init() -> Vec<WorkerGuard> {
    FILES.rotate_log_files().expect("Unabled to rename log files!");

    let color_on_stdout = supports_color::on(Stream::Stdout)
        .map(|support| support.has_basic)
        .unwrap_or(false);
    let (stdio, stdio_guard) = tracing_appender::non_blocking(std::io::stdout());
    let stdout_logger = tracing_subscriber::fmt::layer()
        .event_format(LogFormatter::new(color_on_stdout))
        .with_writer(stdio.with_max_level(max_logging_level()));

    let (latest_log, latest_log_guard) = tracing_appender::non_blocking(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(FILES.latest_log())
            .expect("To be able to open latest.log file"),
    );
    let latest_log_logger = tracing_subscriber::fmt::layer()
        .event_format(LogFormatter::new(false))
        .with_writer(latest_log.with_max_level(max_logging_level()));

    tracing_subscriber::registry()
        .with(stdout_logger)
        .with(latest_log_logger)
        .init();

    vec![stdio_guard, latest_log_guard]
}

pub const fn max_logging_level() -> Level {
    if cfg!(debug_assertions) {
        Level::DEBUG
    } else {
        Level::INFO
    }
}
