use std::{
    fmt::{self, Display, Formatter},
    panic::Location,
};

use tracing_error::{SpanTrace, SpanTraceStatus};

#[derive(Debug, Clone)]
pub struct ErrorDetails {
    // We use Box because SpanTrace is 40 bytes, Box is 8
    // This way we save 32 bytes on the stack
    pub trace: Box<SpanTrace>,
    pub file: &'static str,
    pub line: u32,
}

impl ErrorDetails {
    #[track_caller]
    pub fn new() -> Self {
        let location = Location::caller();
        Self {
            trace: SpanTrace::capture().into(),
            file: location.file(),
            line: location.line(),
        }
    }
}

impl Default for ErrorDetails {
    #[track_caller]
    fn default() -> Self {
        Self::new()
    }
}

impl Display for ErrorDetails {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let trace: &dyn Display = match self.trace.status() {
            SpanTraceStatus::CAPTURED => &self.trace,
            SpanTraceStatus::EMPTY => &"Empty",
            SpanTraceStatus::UNSUPPORTED => &"Trace capture not available",
        };
        write!(f, "At: {}:{}\nTrace: {}", self.file, self.line, trace)
    }
}

pub trait Detailed {
    type Error;

    fn destructure(self) -> (Self::Error, ErrorDetails);
}

// #[cfg(test)]
// mod tests {
//     use std::io::Error;

//     use tracing::{
//         info_span,
//         subscriber::{self},
//     };
//     use tracing_error::ErrorLayer;
//     use tracing_subscriber::{FmtSubscriber, layer::SubscriberExt as _};

//     use crate::error::Detailed;

//     #[test]
//     fn test_display() {
//         let file = file!();
//         let line = line!() + 1;
//         let error: Detailed<_> = Error::other("an IO error occurred").into();
//         let formatted = format!("{}", error);
//         let lines = formatted.lines().collect::<Vec<_>>();
//         assert_eq!(lines[0], "Error: an IO error occurred");
//         assert_eq!(lines[1], &format!("At: {}:{}", file, line));
//     }

//     #[test]
//     fn test_display_no_tracing() {
//         let error: Detailed<_> = Error::other("an IO error occurred").into();
//         let formatted = format!("{}", error);
//         assert_eq!(formatted.lines().nth(2).unwrap(), "Trace: Empty");
//     }

//     #[test]
//     fn test_display_no_span() {
//         subscriber::with_default(FmtSubscriber::new(), || {
//             let error: Detailed<_> = Error::other("an IO error occurred").into();
//             let formatted = format!("{}", error);
//             assert_eq!(formatted.lines().nth(2).unwrap(), "Trace: Empty");
//         });
//     }

//     #[test]
//     fn test_display_unsupported_trace() {
//         subscriber::with_default(FmtSubscriber::new(), || {
//             let _span = info_span!("Test span").entered();

//             let error: Detailed<_> = Error::other("an IO error occurred").into();
//             let formatted = format!("{}", error);
//             assert_eq!(formatted.lines().nth(2).unwrap(), "Trace: Trace capture not available");
//         });
//     }

//     #[test]
//     fn test_display_with_trace() {
//         let subscriber = tracing_subscriber::Registry::default().with(ErrorLayer::default());
//         subscriber::with_default(subscriber, || {
//             let _span = info_span!("Test span").entered();

//             let error: Detailed<_> = Error::other("an IO error occurred").into();
//             let formatted = format!("{}", error);
//             assert_eq!(
//                 formatted.lines().nth(2).unwrap(),
//                 "Trace:    0: crab_utils::error::tests::Test span"
//             );
//         });
//     }
// }
