use std::fmt::{self, Debug};

use chrono::Local;
use nu_ansi_term::Color;
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::{
    fmt::{format::Writer, FmtContext, FormatEvent, FormatFields},
    registry::{LookupSpan, SpanRef},
};

#[derive(Debug, Default, Clone)]
pub struct LogFormatter {
    color_enabled: bool,
}

impl LogFormatter {
    pub fn new(color: bool) -> Self {
        Self { color_enabled: color }
    }
}

impl<S, N> FormatEvent<S, N> for LogFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(&self, ctx: &FmtContext<'_, S, N>, mut writer: Writer, event: &Event) -> fmt::Result {
        let metadata = event.metadata();

        let timestamp = Local::now().format("%H:%M:%S");

        let level = metadata.level();
        let (level, color) = match *level {
            Level::INFO => ("INFO", Color::LightGray),
            Level::WARN => ("WARN", Color::LightYellow),
            Level::ERROR => ("ERROR", Color::LightRed),
            Level::DEBUG => ("DEBUG", Color::LightBlue),
            Level::TRACE => ("TRACE", Color::Blue),
        };

        let span = ctx
            .event_scope()
            .as_mut()
            .and_then(Iterator::next)
            .filter(Self::span_filter);

        let prefix = match span {
            Some(span) => format!("{}/{}", span.name(), level),
            None => level.to_owned(),
        };

        if self.color_enabled {
            write!(
                writer,
                "[{}] [{}] ",
                Color::LightGreen.paint(timestamp.to_string()),
                color.paint(prefix)
            )?;
            // ANSI set color
            write!(writer, "{}", color.prefix())?;
        } else {
            write!(writer, "[{timestamp}] [{prefix}] ")?;
        }

        ctx.field_format().format_fields(writer.by_ref(), event)?;
        if self.color_enabled {
            // ANSI reset color
            write!(writer, "{}", color.suffix())?;
        }

        writeln!(writer)?;
        Ok(())
    }
}

impl LogFormatter {
    fn span_filter<S>(span: &SpanRef<'_, S>) -> bool
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        // Filter out random spans from other crates
        span.metadata()
            .module_path()
            .map(|path| path.starts_with("crab_"))
            .unwrap_or(false)
    }
}
