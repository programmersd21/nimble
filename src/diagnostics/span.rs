use crate::diagnostics::source_map::FileId;

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct DiagnosticSpan {
    pub file_id: FileId,
    pub start: usize, // 0-based byte offset
    pub end: usize,   // 0-based byte offset
    pub expansion_info: Option<crate::diagnostics::span::ExpansionInfoRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ExpansionInfo {
    pub macro_name: String,
    pub def_site: DiagnosticSpan,
    pub call_site: Box<DiagnosticSpan>,
}

// A wrapper to handle recursion in serialized context
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExpansionInfoRef(pub std::sync::Arc<ExpansionInfo>);

impl serde::Serialize for ExpansionInfoRef {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for ExpansionInfoRef {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let inner = ExpansionInfo::deserialize(deserializer)?;
        Ok(ExpansionInfoRef(std::sync::Arc::new(inner)))
    }
}

impl DiagnosticSpan {
    pub fn new(file_id: FileId, start: usize, end: usize) -> Self {
        DiagnosticSpan {
            file_id,
            start,
            end,
            expansion_info: None,
        }
    }

    pub fn with_expansion(mut self, info: ExpansionInfo) -> Self {
        self.expansion_info = Some(ExpansionInfoRef(std::sync::Arc::new(info)));
        self
    }

    /// Convert from the lexer's `Span` type.
    pub fn from_lexer_span(lexer_span: crate::lexer::Span, file_id: FileId) -> Self {
        DiagnosticSpan {
            file_id,
            start: lexer_span.byte_index,
            end: lexer_span.byte_index + lexer_span.length,
            expansion_info: None,
        }
    }

    /// Convert back to the lexer's `Span` type if possible (using line/col lookup from file).
    pub fn to_lexer_span(
        &self,
        file: &crate::diagnostics::source_map::SourceFile,
    ) -> crate::lexer::Span {
        let (line, column) = file.location(self.start);
        crate::lexer::Span {
            line,
            column,
            byte_index: self.start,
            length: self.end.saturating_sub(self.start),
        }
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct MultiSpan {
    pub primary_spans: Vec<DiagnosticSpan>,
    pub secondary_spans: Vec<DiagnosticSpan>,
}

impl MultiSpan {
    pub fn new() -> Self {
        MultiSpan::default()
    }

    pub fn with_primary(mut self, span: DiagnosticSpan) -> Self {
        self.primary_spans.push(span);
        self
    }

    pub fn with_secondary(mut self, span: DiagnosticSpan) -> Self {
        self.secondary_spans.push(span);
        self
    }
}
