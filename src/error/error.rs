use crate::lexer::Span;
use miette::{Diagnostic, GraphicalReportHandler, GraphicalTheme, NamedSource, Report, SourceSpan};
use std::io;
use std::sync::Once;
use thiserror::Error;

pub type NimbleResult<T> = Result<T, Report>;

pub trait GraphicalReportHandlerExt {
    fn with_tab_width(self, width: usize) -> Self;
    fn with_code(self, enabled: bool) -> Self;
}

impl GraphicalReportHandlerExt for GraphicalReportHandler {
    fn with_tab_width(self, width: usize) -> Self {
        self.tab_width(width)
    }

    fn with_code(self, _enabled: bool) -> Self {
        self
    }
}

#[derive(Debug, Clone)]
pub struct SourceFile {
    name: String,
    source: String,
}

impl SourceFile {
    pub fn new(name: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            source: source.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn named_source(&self) -> NamedSource {
        NamedSource::new(self.name.clone(), self.source.clone())
    }
}

#[derive(Debug, Clone)]
pub enum LexErrorKind {
    InvalidToken { found: String },
    UnterminatedString,
    UnterminatedStringEscape,
    InvalidNumber { literal: String },
    Indentation { expected: usize, found: usize },
    Other,
}

#[derive(Debug, Clone)]
pub struct LexError {
    pub kind: LexErrorKind,
    pub message: String,
    pub span: Span,
}

impl LexError {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            kind: LexErrorKind::Other,
            message: message.into(),
            span,
        }
    }

    pub fn invalid_token(found: impl Into<String>, span: Span) -> Self {
        let found = found.into();
        Self {
            kind: LexErrorKind::InvalidToken {
                found: found.clone(),
            },
            message: format!("invalid token `{found}`"),
            span,
        }
    }

    pub fn unterminated_string(span: Span) -> Self {
        Self {
            kind: LexErrorKind::UnterminatedString,
            message: "unterminated string literal".into(),
            span,
        }
    }

    pub fn unterminated_string_escape(span: Span) -> Self {
        Self {
            kind: LexErrorKind::UnterminatedStringEscape,
            message: "unterminated string escape".into(),
            span,
        }
    }

    pub fn invalid_number(literal: impl Into<String>, span: Span) -> Self {
        let literal = literal.into();
        Self {
            kind: LexErrorKind::InvalidNumber {
                literal: literal.clone(),
            },
            message: format!("invalid numeric literal `{literal}`"),
            span,
        }
    }

    pub fn indentation(expected: usize, found: usize, span: Span) -> Self {
        Self {
            kind: LexErrorKind::Indentation { expected, found },
            message: format!("inconsistent indentation: expected {expected} spaces, found {found}"),
            span,
        }
    }
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LexError {}

#[derive(Debug, Clone)]
pub enum ParseErrorKind {
    UnexpectedToken {
        expected: String,
        found: String,
    },
    MissingDelimiter {
        delimiter: String,
        help: Option<String>,
    },
    Other,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub message: String,
    pub span: Span,
}

impl ParseError {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            kind: ParseErrorKind::Other,
            message: message.into(),
            span,
        }
    }

    pub fn unexpected_token(
        expected: impl Into<String>,
        found: impl Into<String>,
        span: Span,
    ) -> Self {
        let expected = expected.into();
        let found = found.into();
        Self {
            kind: ParseErrorKind::UnexpectedToken {
                expected: expected.clone(),
                found: found.clone(),
            },
            message: format!("expected {expected}, found {found}"),
            span,
        }
    }

    pub fn missing_delimiter(
        delimiter: impl Into<String>,
        span: Span,
        help: Option<String>,
    ) -> Self {
        let delimiter = delimiter.into();
        Self {
            kind: ParseErrorKind::MissingDelimiter {
                delimiter: delimiter.clone(),
                help: help.clone(),
            },
            message: format!("missing delimiter `{delimiter}`"),
            span,
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone)]
pub enum SemanticError {
    UndefinedVariable {
        name: String,
        span: Span,
    },
    TypeMismatch {
        expected: String,
        found: String,
        expected_span: Span,
        found_span: Span,
        help: Option<String>,
    },
    Generic {
        message: String,
        span: Span,
        label: String,
        help: Option<String>,
    },
}

#[derive(Debug, Error, Diagnostic)]
pub enum NimbleError {
    #[error("failed to read `{path}`")]
    #[diagnostic(code(io_error))]
    Io {
        path: String,
        #[source]
        source: io::Error,
    },

    #[error("invalid token `{found}`")]
    #[diagnostic(code(lexer_error))]
    InvalidToken {
        found: String,
        #[source_code]
        src: NamedSource,
        #[label("this token is not valid here")]
        span: SourceSpan,
        #[help]
        help: Option<String>,
    },

    #[error("unterminated string literal")]
    #[diagnostic(code(syntax_error))]
    UnterminatedString {
        #[source_code]
        src: NamedSource,
        #[label("string starts here but never closes")]
        span: SourceSpan,
        #[help]
        help: Option<String>,
    },

    #[error("unexpected token")]
    #[diagnostic(code(syntax_error))]
    UnexpectedToken {
        expected: String,
        found: String,
        #[source_code]
        src: NamedSource,
        #[label("expected {expected}, found {found}")]
        span: SourceSpan,
        #[help]
        help: Option<String>,
    },

    #[error("missing delimiter `{delimiter}`")]
    #[diagnostic(code(syntax_error))]
    MissingDelimiter {
        delimiter: String,
        #[source_code]
        src: NamedSource,
        #[label("insert `{delimiter}` here")]
        span: SourceSpan,
        #[help]
        help: Option<String>,
    },

    #[error("type mismatch: expected {expected}, found {found}")]
    #[diagnostic(code(type_error))]
    TypeMismatch {
        expected: String,
        found: String,
        #[source_code]
        src: NamedSource,
        #[label("expected {expected} because of this binding")]
        expected_span: SourceSpan,
        #[label("this expression has type {found}")]
        found_span: SourceSpan,
        #[help]
        help: Option<String>,
    },

    #[error("cannot find `{name}` in this scope")]
    #[diagnostic(code(name_error))]
    UndefinedVariable {
        name: String,
        #[source_code]
        src: NamedSource,
        #[label("`{name}` is not defined here")]
        span: SourceSpan,
        #[help]
        help: Option<String>,
    },

    #[error("{message}")]
    #[diagnostic(code(error))]
    Generic {
        message: String,
        #[source_code]
        src: NamedSource,
        #[label("{label}")]
        span: SourceSpan,
        label: String,
        #[help]
        help: Option<String>,
    },

    #[error("{message}")]
    #[diagnostic(code(multiple_errors))]
    Multiple {
        message: String,
        #[source_code]
        src: NamedSource,
        #[related]
        related: Vec<NimbleError>,
        #[help]
        help: Option<String>,
    },

    #[error("runtime error: {message}")]
    #[diagnostic(code(runtime_error))]
    Runtime {
        message: String,
        #[source_code]
        src: NamedSource,
        #[label("execution failed while running this program")]
        span: SourceSpan,
        #[help]
        help: Option<String>,
    },
}

impl NimbleError {
    pub fn from_lex(source: &SourceFile, error: LexError) -> Self {
        match error.kind {
            LexErrorKind::InvalidToken { found } => Self::InvalidToken {
                found,
                src: source.named_source(),
                span: to_source_span(error.span),
                help: Some("remove the token or replace it with valid Nimble syntax".into()),
            },
            LexErrorKind::UnterminatedString | LexErrorKind::UnterminatedStringEscape => {
                Self::UnterminatedString {
                    src: source.named_source(),
                    span: to_source_span(error.span),
                    help: Some("close the string with a matching quote".into()),
                }
            }
            LexErrorKind::InvalidNumber { .. }
            | LexErrorKind::Indentation { .. }
            | LexErrorKind::Other => Self::Generic {
                message: error.message,
                src: source.named_source(),
                span: to_source_span(error.span),
                label: "lexer error occurred here".into(),
                help: None,
            },
        }
    }

    pub fn from_parse(source: &SourceFile, error: ParseError) -> Self {
        match error.kind {
            ParseErrorKind::UnexpectedToken { expected, found } => Self::UnexpectedToken {
                expected,
                found,
                src: source.named_source(),
                span: to_source_span(error.span),
                help: None,
            },
            ParseErrorKind::MissingDelimiter { delimiter, help } => Self::MissingDelimiter {
                delimiter,
                src: source.named_source(),
                span: to_source_span(error.span),
                help,
            },
            ParseErrorKind::Other => Self::Generic {
                message: error.message,
                src: source.named_source(),
                span: to_source_span(error.span),
                label: "parser error occurred here".into(),
                help: None,
            },
        }
    }

    pub fn from_semantic(source: &SourceFile, error: SemanticError) -> Self {
        match error {
            SemanticError::UndefinedVariable { name, span } => Self::UndefinedVariable {
                name,
                src: source.named_source(),
                span: to_source_span(span),
                help: Some("declare the variable before using it in this scope".into()),
            },
            SemanticError::TypeMismatch {
                expected,
                found,
                expected_span,
                found_span,
                help,
            } => Self::TypeMismatch {
                expected,
                found,
                src: source.named_source(),
                expected_span: to_source_span(expected_span),
                found_span: to_source_span(found_span),
                help,
            },
            SemanticError::Generic {
                message,
                span,
                label,
                help,
            } => Self::Generic {
                message,
                src: source.named_source(),
                span: to_source_span(span),
                label,
                help,
            },
        }
    }

    pub fn runtime(source: &SourceFile, message: impl Into<String>) -> Self {
        Self::Runtime {
            message: message.into(),
            src: source.named_source(),
            span: to_source_span(Span::point(0)),
            help: None,
        }
    }

    pub fn multiple(
        source: &SourceFile,
        message: impl Into<String>,
        related: Vec<NimbleError>,
    ) -> Self {
        Self::Multiple {
            message: message.into(),
            src: source.named_source(),
            related,
            help: None,
        }
    }
}

static DIAGNOSTIC_HOOK: Once = Once::new();

pub fn install_diagnostic_hook() {
    DIAGNOSTIC_HOOK.call_once(|| {
        let _ = miette::set_hook(Box::new(|_| {
            Box::new(
                GraphicalReportHandler::new()
                    .with_theme(GraphicalTheme::unicode())
                    .with_width(80)
                    .with_context_lines(2)
                    .with_tab_width(2)
                    .with_links(false)
                    .with_cause_chain()
                    .with_code(false),
            )
        }));
    });
}

#[derive(Debug, Clone, Copy)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Info,
}

pub fn emit_report(report: &Report) {
    install_diagnostic_hook();
    eprintln!("{report:?}");
}

pub fn print_diagnostic(report: Report, _kind: DiagnosticKind, _summary: Option<&str>) {
    emit_report(&report);
}

pub fn report_for_span(
    name: &str,
    source: &str,
    message: impl Into<String>,
    span: Span,
    label: impl Into<String>,
) -> Report {
    report_for_span_with_help(name, source, message, span, label, None)
}

pub fn report_for_span_with_help(
    name: &str,
    source: &str,
    message: impl Into<String>,
    span: Span,
    label: impl Into<String>,
    help: Option<String>,
) -> Report {
    install_diagnostic_hook();
    Report::new(NimbleError::Generic {
        message: message.into(),
        src: NamedSource::new(name.to_string(), source.to_string()),
        span: to_source_span(span),
        label: label.into(),
        help,
    })
}

pub fn to_source_span(span: Span) -> SourceSpan {
    SourceSpan::new(span.start.into(), span.len.into())
}
