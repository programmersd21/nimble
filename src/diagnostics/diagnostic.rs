use crate::diagnostics::codes::ErrorCode;
use crate::diagnostics::label::Label;
use crate::diagnostics::span::DiagnosticSpan;
use crate::diagnostics::suggestions::Suggestion;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Severity {
    Error,
    Warning,
    Lint,
    Info,
    Note,
    Help,
    Fatal,
    Bug, // Internal Compiler Error (ICE)
    Benchmark,
    OptimizationRemark,
    OptimizationMissed,
    OptimizationSuccess,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Lint => "lint",
            Severity::Info => "info",
            Severity::Note => "note",
            Severity::Help => "help",
            Severity::Fatal => "fatal error",
            Severity::Bug => "internal compiler error",
            Severity::Benchmark => "benchmark",
            Severity::OptimizationRemark => "optimization remark",
            Severity::OptimizationMissed => "optimization missed",
            Severity::OptimizationSuccess => "optimization success",
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RelatedInformation {
    pub span: DiagnosticSpan,
    pub message: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Diagnostic {
    pub code: Option<ErrorCode>,
    pub severity: Severity,
    pub message: String,
    pub primary_spans: Vec<DiagnosticSpan>,
    pub labels: Vec<Label>,
    pub notes: Vec<String>,
    pub helps: Vec<String>,
    pub suggestions: Vec<Suggestion>,
    pub related_info: Vec<RelatedInformation>,
}

impl Diagnostic {
    pub fn new(severity: Severity, code: Option<ErrorCode>, message: impl Into<String>) -> Self {
        Diagnostic {
            code,
            severity,
            message: message.into(),
            primary_spans: Vec::new(),
            labels: Vec::new(),
            notes: Vec::new(),
            helps: Vec::new(),
            suggestions: Vec::new(),
            related_info: Vec::new(),
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(
            self.severity,
            Severity::Error | Severity::Fatal | Severity::Bug
        )
    }
}
