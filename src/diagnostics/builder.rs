use crate::diagnostics::codes::ErrorCode;
use crate::diagnostics::diagnostic::{Diagnostic, RelatedInformation, Severity};
use crate::diagnostics::label::Label;
use crate::diagnostics::span::DiagnosticSpan;
use crate::diagnostics::suggestions::Suggestion;

pub struct DiagnosticBuilder {
    diagnostic: Diagnostic,
}

impl DiagnosticBuilder {
    pub fn new(severity: Severity, message: impl Into<String>) -> Self {
        DiagnosticBuilder {
            diagnostic: Diagnostic::new(severity, None, message),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::new(Severity::Error, message)
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(Severity::Warning, message)
    }

    pub fn lint(message: impl Into<String>) -> Self {
        Self::new(Severity::Lint, message)
    }

    pub fn info(message: impl Into<String>) -> Self {
        Self::new(Severity::Info, message)
    }

    pub fn note_diagnostic(message: impl Into<String>) -> Self {
        Self::new(Severity::Note, message)
    }

    pub fn help_diagnostic(message: impl Into<String>) -> Self {
        Self::new(Severity::Help, message)
    }

    pub fn fatal(message: impl Into<String>) -> Self {
        Self::new(Severity::Fatal, message)
    }

    pub fn bug(message: impl Into<String>) -> Self {
        Self::new(Severity::Bug, message)
    }

    pub fn code(mut self, code: ErrorCode) -> Self {
        self.diagnostic.code = Some(code);
        self
    }

    pub fn primary_span(mut self, span: DiagnosticSpan) -> Self {
        if !self.diagnostic.primary_spans.contains(&span) {
            self.diagnostic.primary_spans.push(span);
        }
        self
    }

    pub fn label(mut self, label: Label) -> Self {
        self.diagnostic.labels.push(label);
        self
    }

    pub fn primary_label(mut self, span: DiagnosticSpan, msg: impl Into<String>) -> Self {
        self.diagnostic
            .labels
            .push(Label::new_primary(span.clone(), msg));
        if !self.diagnostic.primary_spans.contains(&span) {
            self.diagnostic.primary_spans.push(span);
        }
        self
    }

    pub fn secondary_label(mut self, span: DiagnosticSpan, msg: impl Into<String>) -> Self {
        self.diagnostic.labels.push(Label::new_secondary(span, msg));
        self
    }

    pub fn note(mut self, msg: impl Into<String>) -> Self {
        self.diagnostic.notes.push(msg.into());
        self
    }

    pub fn help(mut self, msg: impl Into<String>) -> Self {
        self.diagnostic.helps.push(msg.into());
        self
    }

    pub fn suggestion(mut self, suggestion: Suggestion) -> Self {
        self.diagnostic.suggestions.push(suggestion);
        self
    }

    pub fn related(mut self, span: DiagnosticSpan, msg: impl Into<String>) -> Self {
        self.diagnostic.related_info.push(RelatedInformation {
            span,
            message: msg.into(),
        });
        self
    }

    pub fn build(self) -> Diagnostic {
        self.diagnostic
    }
}
