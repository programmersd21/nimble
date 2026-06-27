use crate::diagnostics::span::DiagnosticSpan;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Label {
    pub span: DiagnosticSpan,
    pub message: String,
    pub is_primary: bool,
}

impl Label {
    pub fn new_primary(span: DiagnosticSpan, message: impl Into<String>) -> Self {
        Label {
            span,
            message: message.into(),
            is_primary: true,
        }
    }

    pub fn new_secondary(span: DiagnosticSpan, message: impl Into<String>) -> Self {
        Label {
            span,
            message: message.into(),
            is_primary: false,
        }
    }
}
