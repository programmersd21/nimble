use colored::{Color, Colorize};
use crate::lexer::Span;
use miette::highlighters::SyntectHighlighter;
use miette::{Diagnostic, GraphicalReportHandler, NamedSource, Report, SourceSpan};
use once_cell::sync::Lazy;
use std::sync::Once;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::SyntaxSet;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct LexError {
    pub message: String,
    pub span: Span,
}

impl LexError {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

impl ParseError {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }
}

#[derive(Debug, Error, Diagnostic)]
#[error("{message}")]
pub struct NimbleDiagnostic {
    message: String,
    #[source_code]
    src: NamedSource<String>,
    #[label("{label}")]
    span: SourceSpan,
    label: String,
    #[help("{help}")]
    help: Option<String>,
}

static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(|| SyntaxSet::load_defaults_newlines());
static THEME: Lazy<Theme> =
    Lazy::new(|| ThemeSet::load_defaults().themes["base16-eighties.dark"].clone());
static DIAGNOSTIC_HOOK: Once = Once::new();

pub fn install_diagnostic_hook() {
    DIAGNOSTIC_HOOK.call_once(|| {
        let syntax = SYNTAX_SET.clone();
        let theme = THEME.clone();
        let _ = miette::set_hook(Box::new(move |_| {
            Box::new(GraphicalReportHandler::new().with_syntax_highlighting(
                SyntectHighlighter::new(syntax.clone(), theme.clone(), true),
            ))
        }));
    });
}

#[derive(Debug, Clone, Copy)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Info,
}

impl DiagnosticKind {
    fn label(self) -> &'static str {
        match self {
            DiagnosticKind::Error => "ERROR",
            DiagnosticKind::Warning => "WARNING",
            DiagnosticKind::Info => "INFO",
        }
    }

    fn color(self) -> Color {
        match self {
            DiagnosticKind::Error => Color::Red,
            DiagnosticKind::Warning => Color::Yellow,
            DiagnosticKind::Info => Color::Cyan,
        }
    }
}

pub fn print_diagnostic(report: Report, kind: DiagnosticKind, summary: Option<&str>) {
    let label = format!("[{}]", kind.label()).color(kind.color()).bold();
    match summary {
        Some(text) => eprintln!("{label} {text}"),
        None => eprintln!("{label} Nimble"),
    }
    eprintln!("{report}");
}

fn ensure_diagnostic_hook() {
    install_diagnostic_hook();
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
    ensure_diagnostic_hook();
    let src = NamedSource::new(name.to_string(), source.to_string()).with_language("nimble");
    let source_span = span_to_source_span(source, span);
    Report::new(NimbleDiagnostic {
        message: message.into(),
        src,
        span: source_span,
        label: label.into(),
        help,
    })
}

fn span_to_source_span(source: &str, span: Span) -> SourceSpan {
    let mut line = 1usize;
    let mut col = 1usize;
    let mut offset = source.len();
    for (idx, ch) in source.char_indices() {
        if line == span.line && col == span.col {
            offset = idx;
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    SourceSpan::new(offset.into(), (span.len.max(1)).into())
}
