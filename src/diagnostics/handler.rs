use std::fmt;
use miette::{Diagnostic, LabeledSpan, ReportHandler, SourceCode, SourceSpan};

/// A miette [`ReportHandler`] that renders diagnostics in Rust-style:
/// `error[N0001]: message` with `--> file:line:col` and `|`-based source annotations.
pub struct RustyHandler;

impl Default for RustyHandler {
    fn default() -> Self {
        Self
    }
}

impl RustyHandler {
    pub fn new() -> Self {
        Self
    }
}

impl ReportHandler for RustyHandler {
    fn debug(&self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            return fmt::Debug::fmt(diagnostic, f);
        }

        let code = diagnostic.code().map(|c| format!("{}", c));
        let msg = format!("{}", diagnostic);

        // 1. Header: error[NXXXX]: message
        if let Some(ref code) = code {
            write!(f, "\x1b[1;31merror[{}]:\x1b[0m {}\n", code, msg)?;
        } else {
            write!(f, "\x1b[1;31merror:\x1b[0m {}\n", msg)?;
        }

        // 2. Source location + annotations
        render_labels(f, diagnostic.source_code(), diagnostic.labels())?;

        // 3. Help
        if let Some(help) = diagnostic.help() {
            let help_str = format!("{}", help);
            write!(f, "\n\x1b[1;32mhelp:\x1b[0m {}\n", help_str)?;
        }

        // 4. Related (notes)
        if let Some(related) = diagnostic.related() {
            for rel in related {
                let rel_msg = format!("{}", rel);
                write!(f, "\x1b[1;34mnote:\x1b[0m {}\n", rel_msg)?;
                render_labels(f, rel.source_code(), rel.labels())?;
            }
        }

        // 5. Code explanation hint
        if let Some(ref code) = code {
            if crate::diagnostics::codes::ErrorCode::parse_str(code).is_some() {
                write!(
                    f,
                    "\n\x1b[38;5;244m= \x1b[0m\x1b[1mhelp:\x1b[0m run `nimble explain {}` for details\n",
                    code
                )?;
            }
        }

        Ok(())
    }
}

fn render_labels(
    f: &mut fmt::Formatter<'_>,
    src: Option<&dyn SourceCode>,
    labels: Option<Box<dyn Iterator<Item = LabeledSpan> + '_>>,
) -> fmt::Result {
    let labels: Vec<LabeledSpan> = match labels {
        Some(it) => it.collect(),
        None => return Ok(()),
    };
    if labels.is_empty() {
        return Ok(());
    }

    let src = match src {
        Some(s) => s,
        None => return Ok(()),
    };

    // Find the full span covering all labels with context
    let min_offset = labels.iter().map(|l| l.inner().offset()).min().unwrap_or(0);
    let max_end = labels
        .iter()
        .map(|l| l.inner().offset() + l.inner().len())
        .max()
        .unwrap_or(1);
    let full_len = max_end - min_offset + 1;

    let full_span = SourceSpan::new(min_offset.into(), full_len);
    let contents = match src.read_span(&full_span, 2, 1) {
        Ok(c) => c,
        Err(_) => return Ok(()),
    };

    let source_bytes = contents.data();
    let source_str = std::str::from_utf8(source_bytes).unwrap_or("");
    let base_line = contents.line();
    let base_col = contents.column();
    let global_offset = min_offset - base_col;

    // Build line starts (relative to the returned source slice)
    let line_starts: Vec<usize> = std::iter::once(0)
        .chain(source_str.match_indices('\n').map(|(i, _)| i + 1))
        .collect();

    let byte_to_line_col = |offset: usize| -> Option<(usize, usize)> {
        let line = line_starts.partition_point(|&s| s <= offset).wrapping_sub(1);
        let col = offset.checked_sub(line_starts.get(line).copied()?)?;
        Some((line, col))
    };

    let line_text = |line: usize| -> Option<&str> {
        let start = *line_starts.get(line)?;
        let end = line_starts.get(line + 1).copied().unwrap_or(source_str.len());
        let text = &source_str[start..end];
        if text.ends_with('\n') || text.ends_with('\r') {
            Some(&text[..text.len() - 1])
        } else {
            Some(text)
        }
    };

    // Group labels by line (in global line numbers)
    let mut line_labels: Vec<(usize, Vec<&LabeledSpan>)> = Vec::new();
    for label in &labels {
        let span = label.inner();
        let abs_offset = span.offset();
        let rel_offset = abs_offset.saturating_sub(global_offset);
        let (rel_start_line, _) = byte_to_line_col(rel_offset).unwrap_or((0, 0));
        let global_start = base_line + rel_start_line;

        let rel_end_offset = rel_offset + span.len().max(1);
        let (rel_end_line, _) = byte_to_line_col(rel_end_offset).unwrap_or((0, 0));
        let global_end = base_line + rel_end_line;

        for gline in global_start..=global_end {
            if let Some(entry) = line_labels.iter_mut().find(|(l, _)| *l == gline) {
                entry.1.push(label);
            } else {
                line_labels.push((gline, vec![label]));
            }
        }
    }
    line_labels.sort_by_key(|(l, _)| *l);

    if line_labels.is_empty() {
        return Ok(());
    }

    // Source name from SpanContents
    let source_name = contents.name().unwrap_or("<source>");
    write!(f, "  \x1b[38;5;244m-->\x1b[0m {}\n", source_name)?;

    let max_line_num = line_labels.last().map(|(l, _)| *l + 1).unwrap_or(0);
    let pad = max_line_num.to_string().len();
    write!(f, "{:pad$} \x1b[38;5;244m|\x1b[0m\n", "", pad = pad)?;

    for &(line_num, ref lbls) in &line_labels {
        let rel_line = line_num.saturating_sub(base_line);
        if let Some(content) = line_text(rel_line) {
            write!(
                f,
                "\x1b[38;5;244m{:>pad$} |\x1b[0m {}\n",
                line_num + 1,
                content,
                pad = pad
            )?;

            // Carets
            let mut carets = vec![' '; content.len().max(1)];
            let mut label_texts: Vec<String> = Vec::new();

            for label in lbls {
                let span = label.inner();
                let abs_offset = span.offset();
                let rel_off = abs_offset.saturating_sub(global_offset);
                let (sl, sc) = byte_to_line_col(rel_off).unwrap_or((0, 0));
                let abs_end = abs_offset + span.len().max(1);
                let rel_end = abs_end.saturating_sub(global_offset);
                let (el, ec) = byte_to_line_col(rel_end).unwrap_or((0, 0));

                let s_global = base_line + sl;
                let e_global = base_line + el;

                if s_global == e_global && s_global == line_num {
                    let from = sc;
                    let to = ec.max(from + 1).min(content.len());
                    for col in from..to {
                        if col < carets.len() {
                            carets[col] = '^';
                        }
                    }
                    if let Some(lbl_txt) = label.label() {
                        label_texts.push(format!("\x1b[1;31m{}\x1b[0m", lbl_txt));
                    }
                } else if s_global == line_num {
                    for col in sc..content.len() {
                        if col < carets.len() {
                            carets[col] = '^';
                        }
                    }
                } else if e_global == line_num {
                    let to = ec.min(content.len());
                    for col in 0..to {
                        if col < carets.len() {
                            carets[col] = '^';
                        }
                    }
                } else if s_global < line_num && e_global > line_num {
                    for col in 0..content.len() {
                        if col < carets.len() {
                            carets[col] = '|';
                        }
                    }
                }
            }

            let caret_str: String = carets.into_iter().collect();
            let trimmed = caret_str.trim_end();
            if !trimmed.is_empty() {
                write!(f, "{:pad$} \x1b[38;5;244m|\x1b[0m \x1b[1;31m{}\x1b[0m", "", trimmed, pad = pad)?;
                if !label_texts.is_empty() {
                    write!(f, " {}", label_texts.join(", "))?;
                }
                writeln!(f)?;
            }
        }
    }

    write!(f, "{:pad$} \x1b[38;5;244m|\x1b[0m\n", "", pad = pad)?;
    Ok(())
}
