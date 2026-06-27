use crate::diagnostics::diagnostic::{Diagnostic, Severity};
use crate::diagnostics::source_map::SourceMap;
use crate::diagnostics::span::DiagnosticSpan;
use tower_lsp::lsp_types as lsp;

pub fn to_lsp_diagnostics(diagnostic: &Diagnostic, source_map: &SourceMap) -> Vec<lsp::Diagnostic> {
    let severity = match diagnostic.severity {
        Severity::Error | Severity::Fatal | Severity::Bug => Some(lsp::DiagnosticSeverity::ERROR),
        Severity::Warning => Some(lsp::DiagnosticSeverity::WARNING),
        Severity::Lint => Some(lsp::DiagnosticSeverity::WARNING),
        Severity::Info
        | Severity::Benchmark
        | Severity::OptimizationRemark
        | Severity::OptimizationMissed
        | Severity::OptimizationSuccess => Some(lsp::DiagnosticSeverity::INFORMATION),
        Severity::Note | Severity::Help => Some(lsp::DiagnosticSeverity::HINT),
    };

    let code = diagnostic
        .code
        .map(|c| lsp::NumberOrString::String(c.as_str().to_string()));

    let mut lsp_diagnostics = Vec::new();

    // Group related info
    let mut related_information = Vec::new();
    for label in &diagnostic.labels {
        if !label.is_primary {
            if let Some(file) = source_map.get(label.span.file_id) {
                let range = span_to_range(&label.span, &file);
                let uri = match lsp::Url::from_file_path(&file.path) {
                    Ok(u) => u,
                    Err(_) => continue,
                };
                related_information.push(lsp::DiagnosticRelatedInformation {
                    location: lsp::Location { uri, range },
                    message: label.message.clone(),
                });
            }
        }
    }

    for rel in &diagnostic.related_info {
        if let Some(file) = source_map.get(rel.span.file_id) {
            let range = span_to_range(&rel.span, &file);
            let uri = match lsp::Url::from_file_path(&file.path) {
                Ok(u) => u,
                Err(_) => continue,
            };
            related_information.push(lsp::DiagnosticRelatedInformation {
                location: lsp::Location { uri, range },
                message: rel.message.clone(),
            });
        }
    }

    // Emit a primary diagnostic for each primary span
    for span in &diagnostic.primary_spans {
        if let Some(file) = source_map.get(span.file_id) {
            let range = span_to_range(span, &file);

            // Build the main message
            let mut msg = diagnostic.message.clone();
            if !diagnostic.notes.is_empty() {
                msg.push_str("\n\nNote:\n");
                for note in &diagnostic.notes {
                    msg.push_str(&format!("* {}\n", note));
                }
            }
            if !diagnostic.helps.is_empty() {
                msg.push_str("\nHelp:\n");
                for help in &diagnostic.helps {
                    msg.push_str(&format!("* {}\n", help));
                }
            }

            lsp_diagnostics.push(lsp::Diagnostic {
                range,
                severity,
                code: code.clone(),
                code_description: None,
                source: Some("nimble".to_string()),
                message: msg,
                related_information: if related_information.is_empty() {
                    None
                } else {
                    Some(related_information.clone())
                },
                tags: None,
                data: None,
            });
        }
    }

    // Fallback if no primary spans exist
    if lsp_diagnostics.is_empty() {
        lsp_diagnostics.push(lsp::Diagnostic {
            range: lsp::Range::default(),
            severity,
            code,
            code_description: None,
            source: Some("nimble".to_string()),
            message: diagnostic.message.clone(),
            related_information: None,
            tags: None,
            data: None,
        });
    }

    lsp_diagnostics
}

fn span_to_range(
    span: &DiagnosticSpan,
    file: &crate::diagnostics::source_map::SourceFile,
) -> lsp::Range {
    let (start_line, start_col) = file.location(span.start);
    let (end_line, end_col) = file.location(span.end);
    lsp::Range {
        start: lsp::Position::new(
            (start_line.saturating_sub(1)) as u32,
            (start_col.saturating_sub(1)) as u32,
        ),
        end: lsp::Position::new(
            (end_line.saturating_sub(1)) as u32,
            (end_col.saturating_sub(1)) as u32,
        ),
    }
}
