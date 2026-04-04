pub mod error;
pub use error::{
    install_diagnostic_hook, print_diagnostic, report_for_span, DiagnosticKind, LexError,
    NimbleDiagnostic, ParseError,
};
