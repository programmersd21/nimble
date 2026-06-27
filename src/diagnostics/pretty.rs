use crate::diagnostics::diagnostic::{Diagnostic, Severity};
use crate::diagnostics::source_map::{FileId, SourceMap};
use crate::diagnostics::theme::Theme;
use std::collections::BTreeMap;

pub fn render_diagnostic(diagnostic: &Diagnostic, source_map: &SourceMap) -> String {
    let theme = Theme::new();
    let mut out = String::new();

    // 1. Header: severity[code]: message
    let sev_color = match diagnostic.severity {
        Severity::Error | Severity::Fatal => theme.error,
        Severity::Bug => theme.bug,
        Severity::Warning => theme.warning,
        Severity::Lint => theme.lint,
        Severity::Info => theme.info,
        Severity::Note => theme.note,
        Severity::Help => theme.help,
        Severity::Benchmark
        | Severity::OptimizationRemark
        | Severity::OptimizationMissed
        | Severity::OptimizationSuccess => theme.opt_remark,
    };

    let code_str = if let Some(code) = diagnostic.code {
        format!("[{}]", code.as_str())
    } else {
        "".to_string()
    };

    out.push_str(&format!(
        "{}{}{}{}: {}{}{}\n",
        sev_color,
        theme.bold,
        diagnostic.severity.as_str(),
        code_str,
        theme.reset,
        theme.bold,
        diagnostic.message
    ));

    // 2. Group labels by FileId
    let mut file_spans: BTreeMap<FileId, Vec<crate::diagnostics::label::Label>> = BTreeMap::new();
    for label in &diagnostic.labels {
        file_spans
            .entry(label.span.file_id)
            .or_default()
            .push(label.clone());
    }

    // If no labels exist but we have primary spans, group them
    if diagnostic.labels.is_empty() {
        for span in &diagnostic.primary_spans {
            file_spans.entry(span.file_id).or_default().push(
                crate::diagnostics::label::Label::new_primary(span.clone(), ""),
            );
        }
    }

    for (file_id, mut labels) in file_spans {
        let file = match source_map.get(file_id) {
            Some(f) => f,
            None => continue,
        };

        // Sort labels by starting byte offset
        labels.sort_by_key(|l| l.span.start);

        // Find primary or first span to print the source location header
        let head_span = labels
            .iter()
            .find(|l| l.is_primary)
            .map(|l| l.span.clone())
            .unwrap_or_else(|| labels[0].span.clone());

        let (head_line, head_col) = file.location(head_span.start);
        out.push_str(&format!(
            "{}-->{} {}:{}:{}\n",
            theme.border,
            theme.reset,
            file.path.display(),
            head_line,
            head_col
        ));

        // Group labels by line range
        let mut line_to_labels: BTreeMap<usize, Vec<crate::diagnostics::label::Label>> =
            BTreeMap::new();
        for label in &labels {
            let (start_line, _) = file.location(label.span.start);
            let (end_line, _) =
                file.location(label.span.end.saturating_sub(1).max(label.span.start));
            for line in start_line..=end_line {
                line_to_labels.entry(line).or_default().push(label.clone());
            }
        }

        // Print segments of source code
        if !line_to_labels.is_empty() {
            let max_line_num = *line_to_labels.keys().max().unwrap();
            let pad_len = max_line_num.to_string().len();

            out.push_str(&format!("{:pad$} {} |\n", "", theme.border, pad = pad_len));

            // To avoid printing separate lines that are far apart, we could partition them.
            // For simplicity and completeness, we iterate through lines, and print a separator `...` if gap > 2.
            let mut last_printed_line = None;
            for (&line_num, line_labels) in &line_to_labels {
                if let Some(last) = last_printed_line {
                    if line_num > last + 1 {
                        if line_num > last + 2 {
                            out.push_str(&format!("... |\n"));
                        } else {
                            // Print intermediate line without labels
                            let inter_line = last + 1;
                            if let Some(content) = file.get_line(inter_line - 1) {
                                out.push_str(&format!(
                                    "{:pad$} {} | {}\n",
                                    inter_line,
                                    theme.border,
                                    content,
                                    pad = pad_len
                                ));
                            }
                        }
                    }
                }

                if let Some(content) = file.get_line(line_num - 1) {
                    out.push_str(&format!(
                        "{:pad$}{} |{} {}\n",
                        line_num,
                        theme.border,
                        theme.reset,
                        content,
                        pad = pad_len
                    ));

                    // Print underlining carets/markers below the source line
                    let mut caret_line = format!("{:pad$} {} | ", "", theme.border, pad = pad_len);
                    let mut has_carets = false;

                    // Build caret annotation matching the columns
                    let line_len = content.len();
                    let mut column_markers = vec![' '; line_len + 1];

                    // Draw labels for the line
                    let mut label_text_to_append = Vec::new();
                    for label in line_labels {
                        let (start_l, start_c) = file.location(label.span.start);
                        let (end_l, end_c) =
                            file.location(label.span.end.saturating_sub(1).max(label.span.start));

                        let label_color = if label.is_primary {
                            theme.primary_label
                        } else {
                            theme.secondary_label
                        };
                        let marker_char = if label.is_primary { '^' } else { '-' };

                        if start_l == end_l && start_l == line_num {
                            // Single line span
                            let from_col = start_c.saturating_sub(1);
                            let to_col = end_c.saturating_sub(1).max(from_col + 1);
                            for col in from_col..to_col {
                                if col < column_markers.len() {
                                    column_markers[col] = marker_char;
                                }
                            }
                            if !label.message.is_empty() {
                                label_text_to_append.push((
                                    from_col,
                                    format!("{}{}{}", label_color, label.message, theme.reset),
                                ));
                            }
                            has_carets = true;
                        } else {
                            // Multi-line span
                            if line_num == start_l {
                                // Start of multi-line
                                let from_col = start_c.saturating_sub(1);
                                for col in from_col..line_len {
                                    if col < column_markers.len() {
                                        column_markers[col] = '/';
                                    }
                                }
                                has_carets = true;
                            } else if line_num == end_l {
                                // End of multi-line
                                let to_col = end_c.saturating_sub(1).max(1);
                                for col in 0..to_col {
                                    if col < column_markers.len() {
                                        column_markers[col] = '\\';
                                    }
                                }
                                if !label.message.is_empty() {
                                    label_text_to_append.push((
                                        0,
                                        format!("{}{}{}", label_color, label.message, theme.reset),
                                    ));
                                }
                                has_carets = true;
                            } else {
                                // Middle of multi-line
                                for col in 0..line_len {
                                    if col < column_markers.len() {
                                        column_markers[col] = '|';
                                    }
                                }
                                has_carets = true;
                            }
                        }
                    }

                    if has_carets {
                        // Colorize carets
                        for (_col, &ch) in column_markers.iter().enumerate() {
                            if ch != ' ' {
                                let c_color = if ch == '^' {
                                    theme.primary_label
                                } else {
                                    theme.secondary_label
                                };
                                caret_line.push_str(c_color);
                                caret_line.push(ch);
                                caret_line.push_str(theme.reset);
                            } else {
                                caret_line.push(' ');
                            }
                        }

                        // Append label texts if any
                        if !label_text_to_append.is_empty() {
                            // Sort by column position so they align correctly
                            label_text_to_append.sort_by_key(|t| t.0);
                            let messages: Vec<String> =
                                label_text_to_append.into_iter().map(|t| t.1).collect();
                            caret_line.push_str(" ");
                            caret_line.push_str(&messages.join(", "));
                        }

                        out.push_str(&caret_line);
                        out.push('\n');
                    }
                }
                last_printed_line = Some(line_num);
            }
            out.push_str(&format!("{:pad$} {} |\n", "", theme.border, pad = pad_len));
        }

        // Show macro expansion trace if expansion info is present
        for label in &labels {
            if let Some(ref exp) = label.span.expansion_info {
                out.push_str(&format!(
                    "{}note:{} this error originates in the macro `{}`\n",
                    theme.info, theme.reset, exp.0.macro_name
                ));
                // Optional recursive call-site dump
                let call_file = source_map.get(exp.0.call_site.file_id);
                if let Some(cf) = call_file {
                    let (cl, cc) = cf.location(exp.0.call_site.start);
                    out.push_str(&format!(
                        "  --> {}:{}:{} (macro expansion call-site)\n",
                        cf.path.display(),
                        cl,
                        cc
                    ));
                }
            }
        }
    }

    // 3. Suggestions rendering
    for sugg in &diagnostic.suggestions {
        out.push_str(&format!(
            "{}help:{} {}\n",
            theme.help, theme.reset, sugg.msg
        ));

        // Group replacements by FileId
        let mut sugg_files: BTreeMap<FileId, Vec<crate::diagnostics::suggestions::FixIt>> =
            BTreeMap::new();
        for sub in &sugg.substitutions {
            sugg_files
                .entry(sub.span.file_id)
                .or_default()
                .push(sub.clone());
        }

        for (file_id, subs) in sugg_files {
            let file = match source_map.get(file_id) {
                Some(f) => f,
                None => continue,
            };

            // For simple inline additions/replacements, we can show a diff-like view
            for sub in &subs {
                let (line_num, _) = file.location(sub.span.start);
                if let Some(original_line) = file.get_line(line_num - 1) {
                    let pad_len = line_num.to_string().len();

                    // Reconstruct suggestion line
                    let mut suggested_line = original_line.to_string();
                    let offset_in_line = sub
                        .span
                        .start
                        .saturating_sub(file.line_starts[line_num - 1]);
                    let len_in_line = sub.span.end.saturating_sub(sub.span.start);

                    if offset_in_line <= suggested_line.len() {
                        let replace_end = (offset_in_line + len_in_line).min(suggested_line.len());
                        suggested_line.replace_range(offset_in_line..replace_end, &sub.replacement);

                        out.push_str(&format!(
                            "{:pad$}{} |{} {}\n",
                            line_num,
                            theme.border,
                            theme.reset,
                            suggested_line,
                            pad = pad_len
                        ));

                        // Draw green `+` indicators under modified columns
                        let mut plus_line =
                            format!("{:pad$} {} | ", "", theme.border, pad = pad_len);
                        for i in 0..suggested_line.len() {
                            if i >= offset_in_line && i < offset_in_line + sub.replacement.len() {
                                plus_line.push_str(&format!("\x1b[1;32m+\x1b[0m"));
                            } else {
                                plus_line.push(' ');
                            }
                        }
                        out.push_str(&plus_line);
                        out.push('\n');
                    }
                }
            }
        }
    }

    // 4. Notes
    for note in &diagnostic.notes {
        out.push_str(&format!("{}note:{} {}\n", theme.note, theme.reset, note));
    }

    // 5. Helps
    for help in &diagnostic.helps {
        out.push_str(&format!("{}help:{} {}\n", theme.help, theme.reset, help));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::builder::DiagnosticBuilder;
    use crate::diagnostics::codes::ErrorCode;
    use crate::diagnostics::json::to_json_string;
    use crate::diagnostics::lsp::to_lsp_diagnostics;
    use crate::diagnostics::source_map::SourceMap;
    use crate::diagnostics::span::DiagnosticSpan;
    use crate::diagnostics::suggestions::{Applicability, FixIt, Suggestion};
    use std::path::PathBuf;

    #[test]
    fn test_diagnostics_suite() {
        let source_map = SourceMap::new();
        let path = PathBuf::from("test.nim");
        let source = "let x = foo()\nlet y = 123\n".to_string();
        let file_id = source_map.insert(path, source);

        // Span 1: "foo()" (from offset 8 to 13)
        let span1 = DiagnosticSpan::new(file_id, 8, 13);
        // Span 2: "let y = 123" (from offset 14 to 25)
        let span2 = DiagnosticSpan::new(file_id, 14, 25);

        let diag = DiagnosticBuilder::error("expected ';' after expression")
            .code(ErrorCode::N1004)
            .primary_label(span1.clone(), "expected ';' here")
            .secondary_label(span2, "other statement begins here")
            .note("variables declared with let are immutable")
            .suggestion(Suggestion::new(
                "add ';'",
                Applicability::MachineApplicable,
                vec![FixIt::new(span1, "foo();")],
            ))
            .build();

        // 1. Test Pretty printing (Render)
        let rendered = render_diagnostic(&diag, &source_map);
        assert!(rendered.contains("error"));
        assert!(rendered.contains("N1004"));
        assert!(rendered.contains("expected ';' after expression"));
        assert!(rendered.contains("expected ';' here"));
        assert!(rendered.contains("other statement begins here"));
        assert!(rendered.contains("variables declared with let are immutable"));
        assert!(rendered.contains("help"));
        assert!(rendered.contains("add ';'"));

        // 2. Test JSON emission
        let json_str = to_json_string(&diag, &source_map);
        assert!(json_str.contains("expected ';' after expression"));
        assert!(json_str.contains("N1004"));
        assert!(json_str.contains("MachineApplicable"));

        // 3. Test LSP conversion
        let lsp_diags = to_lsp_diagnostics(&diag, &source_map);
        assert_eq!(lsp_diags.len(), 1);
        assert_eq!(
            lsp_diags[0].code,
            Some(tower_lsp::lsp_types::NumberOrString::String(
                "N1004".to_string()
            ))
        );
        assert!(
            lsp_diags[0]
                .message
                .contains("expected ';' after expression")
        );
    }
}
