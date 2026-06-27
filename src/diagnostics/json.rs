use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::source_map::SourceMap;

#[derive(serde::Serialize)]
struct JsonDiagnostic {
    message: String,
    code: Option<String>,
    level: &'static str,
    spans: Vec<JsonSpan>,
    children: Vec<JsonDiagnosticChild>,
    suggestions: Vec<JsonSuggestion>,
}

#[derive(serde::Serialize)]
struct JsonSpan {
    file_name: String,
    byte_start: usize,
    byte_end: usize,
    line_start: usize,
    line_end: usize,
    column_start: usize,
    column_end: usize,
    is_primary: bool,
    label: Option<String>,
}

#[derive(serde::Serialize)]
struct JsonDiagnosticChild {
    message: String,
    level: &'static str,
    spans: Vec<JsonSpan>,
}

#[derive(serde::Serialize)]
struct JsonSuggestion {
    message: String,
    applicability: &'static str,
    substitutions: Vec<JsonSubstitution>,
}

#[derive(serde::Serialize)]
struct JsonSubstitution {
    file_name: String,
    byte_start: usize,
    byte_end: usize,
    replacement: String,
}

pub fn to_json_string(diagnostic: &Diagnostic, source_map: &SourceMap) -> String {
    let code = diagnostic.code.map(|c| c.as_str().to_string());

    // Map primary and secondary spans (labels)
    let mut spans = Vec::new();
    for label in &diagnostic.labels {
        if let Some(file) = source_map.get(label.span.file_id) {
            let (l_start, c_start) = file.location(label.span.start);
            let (l_end, c_end) = file.location(label.span.end);
            spans.push(JsonSpan {
                file_name: file.path.to_string_lossy().to_string(),
                byte_start: label.span.start,
                byte_end: label.span.end,
                line_start: l_start,
                line_end: l_end,
                column_start: c_start,
                column_end: c_end,
                is_primary: label.is_primary,
                label: Some(label.message.clone()),
            });
        }
    }

    // Map children (notes and helps)
    let mut children = Vec::new();
    for note in &diagnostic.notes {
        children.push(JsonDiagnosticChild {
            message: note.clone(),
            level: "note",
            spans: Vec::new(),
        });
    }
    for help in &diagnostic.helps {
        children.push(JsonDiagnosticChild {
            message: help.clone(),
            level: "help",
            spans: Vec::new(),
        });
    }

    // Map suggestions
    let mut suggestions = Vec::new();
    for sugg in &diagnostic.suggestions {
        let app_str = match sugg.applicability {
            crate::diagnostics::suggestions::Applicability::MachineApplicable => {
                "MachineApplicable"
            }
            crate::diagnostics::suggestions::Applicability::MaybeIncorrect => "MaybeIncorrect",
            crate::diagnostics::suggestions::Applicability::HasPlaceholders => "HasPlaceholders",
            crate::diagnostics::suggestions::Applicability::Unspecified => "Unspecified",
        };
        let mut subs = Vec::new();
        for fix in &sugg.substitutions {
            if let Some(file) = source_map.get(fix.span.file_id) {
                subs.push(JsonSubstitution {
                    file_name: file.path.to_string_lossy().to_string(),
                    byte_start: fix.span.start,
                    byte_end: fix.span.end,
                    replacement: fix.replacement.clone(),
                });
            }
        }
        suggestions.push(JsonSuggestion {
            message: sugg.msg.clone(),
            applicability: app_str,
            substitutions: subs,
        });
    }

    let json_diag = JsonDiagnostic {
        message: diagnostic.message.clone(),
        code,
        level: diagnostic.severity.as_str(),
        spans,
        children,
        suggestions,
    };

    serde_json::to_string(&json_diag).unwrap_or_else(|_| "{}".to_string()) + "\n"
}
