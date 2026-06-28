pub mod builder;
pub mod cache;
pub mod codes;
pub mod diagnostic;
pub mod json;
pub mod label;
pub mod lsp;
pub mod pretty;
pub mod recovery;
pub mod renderer;
pub mod source_map;
pub mod span;
pub mod suggestions;
pub mod theme;

use crate::diagnostics::builder::DiagnosticBuilder;
use crate::diagnostics::cache::DiagnosticCache;
use crate::diagnostics::codes::ErrorCode;
use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::recovery::RecoveryState;
use crate::diagnostics::renderer::DiagnosticRenderer;
use crate::diagnostics::source_map::{FileId, SourceMap};
use crate::diagnostics::span::DiagnosticSpan;
use crate::diagnostics::suggestions::{Applicability, FixIt, Suggestion};
use std::sync::{Arc, Mutex};

use crate::errors::{LexError, ParseError, ResolveError};
use crate::typechecker::TypeError;

pub struct DiagnosticEngine {
    pub source_map: Arc<SourceMap>,
    pub cache: Arc<Mutex<DiagnosticCache>>,
    pub recovery: Arc<Mutex<RecoveryState>>,
    pub emitted: Arc<Mutex<Vec<Diagnostic>>>,
    pub json_mode: Mutex<bool>,
}

impl Default for DiagnosticEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl DiagnosticEngine {
    pub fn new() -> Self {
        DiagnosticEngine {
            source_map: Arc::new(SourceMap::new()),
            cache: Arc::new(Mutex::new(DiagnosticCache::new())),
            recovery: Arc::new(Mutex::new(RecoveryState::new())),
            emitted: Arc::new(Mutex::new(Vec::new())),
            json_mode: Mutex::new(false),
        }
    }

    pub fn set_json_mode(&self, enabled: bool) {
        let mut mode = self.json_mode.lock().unwrap();
        *mode = enabled;
    }

    pub fn is_json_mode(&self) -> bool {
        *self.json_mode.lock().unwrap()
    }

    pub fn emit(&self, diagnostic: Diagnostic) {
        // Suppression check
        {
            let rec = self.recovery.lock().unwrap();
            if rec.should_suppress(&diagnostic) {
                return;
            }
        }

        // Insert to cache for deduplication
        let is_new = {
            let mut cache = self.cache.lock().unwrap();
            cache.insert(diagnostic.clone())
        };

        if is_new {
            // Add to emitted
            {
                let mut em = self.emitted.lock().unwrap();
                em.push(diagnostic.clone());
            }

            // Print to console
            let mode = self.is_json_mode();
            let renderer = DiagnosticRenderer::new(&self.source_map, mode);
            renderer.emit(&diagnostic);
        }
    }

    pub fn get_emitted(&self) -> Vec<Diagnostic> {
        let em = self.emitted.lock().unwrap();
        em.clone()
    }

    pub fn clear(&self) {
        self.emitted.lock().unwrap().clear();
        self.cache.lock().unwrap().clear();
    }

    /// Converts a legacy LexError to our structured Diagnostic and emits it
    pub fn emit_lex_error(&self, err: &LexError, file_id: FileId, source: &str) {
        let diag = self.convert_lex_error(err, file_id, source);
        self.emit(diag);
    }

    /// Converts a legacy ParseError to our structured Diagnostic and emits it
    pub fn emit_parse_error(&self, err: &ParseError, file_id: FileId, source: &str) {
        let diag = self.convert_parse_error(err, file_id, source);
        self.emit(diag);
    }

    /// Converts a legacy ResolveError to our structured Diagnostic and emits it
    pub fn emit_resolve_error(&self, err: &ResolveError, file_id: FileId, source: &str) {
        let diag = self.convert_resolve_error(err, file_id, source);
        self.emit(diag);
    }

    /// Converts a legacy TypeError to our structured Diagnostic and emits it
    pub fn emit_type_error(&self, err: &TypeError, file_id: FileId, source: &str) {
        let diag = self.convert_type_error(err, file_id, source);
        self.emit(diag);
    }

    pub fn convert_lex_error(&self, err: &LexError, file_id: FileId, _source: &str) -> Diagnostic {
        let span = err.span();
        let diag_span =
            DiagnosticSpan::new(file_id, span.byte_index, span.byte_index + span.length);

        match err {
            LexError::IllegalTab { help, .. } => {
                let mut builder = DiagnosticBuilder::error("Illegal tab character")
                    .code(ErrorCode::N0001)
                    .primary_label(diag_span.clone(), "tabs are not allowed")
                    .help(*help);

                // Add replacement suggestion (tab to spaces)
                builder = builder.suggestion(Suggestion::new(
                    "replace tab with spaces",
                    Applicability::MachineApplicable,
                    vec![FixIt::new(diag_span, "    ")],
                ));

                builder.build()
            }
            LexError::UnexpectedCharacter { ch, .. } => {
                DiagnosticBuilder::error(format!("Unexpected character `{}`", ch))
                    .code(ErrorCode::N0002)
                    .primary_label(diag_span, format!("unexpected character `{}`", ch))
                    .build()
            }
            LexError::UnmatchedDelimiter {
                delimiter, help, ..
            } => DiagnosticBuilder::error(format!("Unmatched closing delimiter `{}`", delimiter))
                .code(ErrorCode::N0003)
                .primary_label(diag_span, "no matching opening delimiter")
                .help(*help)
                .build(),
            LexError::InvalidFloat { literal, .. } => {
                DiagnosticBuilder::error(format!("Invalid float literal `{}`", literal))
                    .code(ErrorCode::N0004)
                    .primary_label(diag_span, "invalid float")
                    .build()
            }
            LexError::IntOverflow { literal, help, .. } => {
                DiagnosticBuilder::error(format!("Integer literal `{}` out of range", literal))
                    .code(ErrorCode::N0005)
                    .primary_label(diag_span, "integer value too large")
                    .help(*help)
                    .build()
            }
            LexError::UnterminatedString { help, .. } => {
                DiagnosticBuilder::error("Unterminated string literal")
                    .code(ErrorCode::N0006)
                    .primary_label(diag_span, "missing closing `\"`")
                    .help(*help)
                    .build()
            }
            LexError::InvalidEscape { escape, help, .. } => {
                DiagnosticBuilder::error(format!("Invalid escape sequence `\\{}`", escape))
                    .code(ErrorCode::N0007)
                    .primary_label(diag_span, "invalid escape")
                    .help(*help)
                    .build()
            }
            LexError::NewlineInString { help, .. } => {
                DiagnosticBuilder::error("Newline inside string literal")
                    .code(ErrorCode::N0008)
                    .primary_label(diag_span, "newline inside string")
                    .help(*help)
                    .build()
            }
            LexError::IndentationError { indent, help, .. } => DiagnosticBuilder::error(format!(
                "Indentation error: indent level {} does not match any enclosing block",
                indent
            ))
            .code(ErrorCode::N0009)
            .primary_label(
                diag_span,
                format!("indent level {} is inconsistent", indent),
            )
            .help(*help)
            .build(),
        }
    }

    pub fn convert_parse_error(
        &self,
        err: &ParseError,
        file_id: FileId,
        _source: &str,
    ) -> Diagnostic {
        let span = err.span();
        let diag_span =
            DiagnosticSpan::new(file_id, span.byte_index, span.byte_index + span.length);

        match err {
            ParseError::ExpectedToken {
                expected, found, ..
            } => {
                let mut builder = DiagnosticBuilder::error(format!(
                    "Expected `{}` but found `{}`",
                    expected, found
                ))
                .code(ErrorCode::N1001)
                .primary_label(diag_span.clone(), format!("expected `{}`", expected));

                // Contextual help for common missing tokens like semicolon
                if expected == "';'" {
                    builder = builder.suggestion(Suggestion::new(
                        "add ';'",
                        Applicability::MachineApplicable,
                        vec![FixIt::new(diag_span, ";")],
                    ));
                }
                builder.build()
            }
            ParseError::UnexpectedToken { found, .. } => {
                DiagnosticBuilder::error(format!("Unexpected token `{}`", found))
                    .code(ErrorCode::N1002)
                    .primary_label(diag_span, format!("unexpected `{}`", found))
                    .build()
            }
            ParseError::ExpectedExpression { .. } => {
                DiagnosticBuilder::error("Expected expression")
                    .code(ErrorCode::N1003)
                    .primary_label(diag_span, "expected expression")
                    .build()
            }
            ParseError::UnclosedParen { .. } => DiagnosticBuilder::error("Unclosed `(`")
                .code(ErrorCode::N1004)
                .primary_label(diag_span, "unclosed `(`")
                .build(),
            ParseError::ExpectedIndentedBlock { .. } => {
                DiagnosticBuilder::error("Expected indented block after `:`")
                    .code(ErrorCode::N1005)
                    .primary_label(diag_span, "expected indented block")
                    .build()
            }
            ParseError::UnexpectedIndent { .. } => {
                DiagnosticBuilder::error("Unexpected indentation")
                    .code(ErrorCode::N1006)
                    .primary_label(diag_span, "unexpected indentation")
                    .build()
            }
            ParseError::ExpectedIdentifier { .. } => {
                DiagnosticBuilder::error("Expected identifier")
                    .code(ErrorCode::N1007)
                    .primary_label(diag_span, "expected identifier")
                    .build()
            }
            ParseError::ExpectedType { .. } => DiagnosticBuilder::error("Expected type name")
                .code(ErrorCode::N1008)
                .primary_label(diag_span, "expected type name")
                .build(),
            ParseError::ExpectedParameter { .. } => {
                DiagnosticBuilder::error("Expected parameter name")
                    .code(ErrorCode::N1009)
                    .primary_label(diag_span, "expected parameter name")
                    .build()
            }
            ParseError::Internal { msg, .. } => {
                DiagnosticBuilder::bug(format!("Internal error: {}", msg))
                    .primary_label(diag_span, "internal error occurred here")
                    .build()
            }
        }
    }

    pub fn convert_resolve_error(
        &self,
        err: &ResolveError,
        file_id: FileId,
        _source: &str,
    ) -> Diagnostic {
        match err {
            ResolveError::UndefinedVariable { name, span, .. } => {
                let diag_span =
                    DiagnosticSpan::new(file_id, span.offset(), span.offset() + span.len());
                let mut builder =
                    DiagnosticBuilder::error(format!("Undefined variable `{}`", name))
                        .code(ErrorCode::N2001)
                        .primary_label(
                            diag_span.clone(),
                            format!("`{}` is not defined in this scope", name),
                        );

                // Find typo suggestions in scope
                let candidates = vec![
                    "println", "print", "printf", "let", "var", "fn", "if", "while",
                ];
                let suggestions = suggestions::get_spelling_suggestions(name, &candidates, 3);
                if !suggestions.is_empty() {
                    let best = &suggestions[0].0;
                    builder = builder
                        .help(format!("did you mean `{}`?", best))
                        .suggestion(Suggestion::new(
                            format!("change `{}` to `{}`", name, best),
                            Applicability::MaybeIncorrect,
                            vec![FixIt::new(diag_span, best.clone())],
                        ));
                }
                builder.build()
            }
            ResolveError::DuplicateDefinition {
                name,
                existing_span,
                new_span,
                ..
            } => {
                let primary = DiagnosticSpan::new(
                    file_id,
                    new_span.byte_index,
                    new_span.byte_index + new_span.length,
                );
                let secondary = DiagnosticSpan::new(
                    file_id,
                    existing_span.byte_index,
                    existing_span.byte_index + existing_span.length,
                );
                DiagnosticBuilder::error(format!("Duplicate definition of `{}`", name))
                    .code(ErrorCode::N2002)
                    .primary_label(primary, format!("`{}` is already defined", name))
                    .secondary_label(secondary, "original definition was here")
                    .build()
            }
        }
    }

    pub fn convert_type_error(
        &self,
        err: &TypeError,
        file_id: FileId,
        _source: &str,
    ) -> Diagnostic {
        let span = err.span();
        let diag_span =
            DiagnosticSpan::new(file_id, span.byte_index, span.byte_index + span.length);

        match err {
            TypeError::Mismatch {
                expected, found, ..
            } => {
                let mut builder = DiagnosticBuilder::error(format!(
                    "Type mismatch: expected `{}`, found `{}`",
                    expected, found
                ))
                .code(ErrorCode::N3001)
                .primary_label(
                    diag_span.clone(),
                    format!("expected `{}`, found `{}`", expected, found),
                );

                // Handcrafted type error explanations
                if expected == "i32" && found == "String" {
                    builder = builder.note("expected integer because this parameter/operation expects a numeric value")
                        .note("found string because this expression evaluates to text");
                } else if expected == "String" && found == "i32" {
                    builder = builder
                        .note("expected string because this parameter/operation expects text")
                        .note("found integer because this expression evaluates to a numeric value");
                }
                builder.build()
            }
            TypeError::AssignToImmutable { name, .. } => DiagnosticBuilder::error(format!(
                "Cannot reassign to immutable variable `{}`",
                name
            ))
            .code(ErrorCode::N3002)
            .primary_label(
                diag_span.clone(),
                format!("`{}` is declared as `let`", name),
            )
            .note("variables declared with let are immutable")
            .help("use `var` or `mut` if mutation is intended")
            .build(),
            TypeError::UndefinedVariable { name, .. } => {
                let mut builder =
                    DiagnosticBuilder::error(format!("Undefined variable `{}`", name))
                        .code(ErrorCode::N2001)
                        .primary_label(diag_span.clone(), format!("undefined `{}`", name));

                let candidates = vec![
                    "println", "print", "printf", "let", "var", "fn", "if", "while",
                ];
                let suggestions = suggestions::get_spelling_suggestions(name, &candidates, 3);
                if !suggestions.is_empty() {
                    let best = &suggestions[0].0;
                    builder = builder
                        .help(format!("did you mean `{}`?", best))
                        .suggestion(Suggestion::new(
                            format!("change `{}` to `{}`", name, best),
                            Applicability::MaybeIncorrect,
                            vec![FixIt::new(diag_span, best.clone())],
                        ));
                }
                builder.build()
            }
            TypeError::UndefinedType { name, .. } => {
                let mut builder = DiagnosticBuilder::error(format!("Undefined type `{}`", name))
                    .code(ErrorCode::N3003)
                    .primary_label(diag_span, format!("undefined type `{}`", name));

                let candidates = vec!["i32", "Float", "String", "Bool", "Void"];
                let suggestions = suggestions::get_spelling_suggestions(name, &candidates, 3);
                if !suggestions.is_empty() {
                    let best = &suggestions[0].0;
                    builder = builder.help(format!("did you mean `{}`?", best));
                }
                builder.build()
            }
            TypeError::DuplicateDefinition { name, .. } => {
                DiagnosticBuilder::error(format!("Duplicate definition `{}`", name))
                    .code(ErrorCode::N2002)
                    .primary_label(
                        diag_span,
                        format!("`{}` already defined in this scope", name),
                    )
                    .build()
            }
            TypeError::CallNonFunction { .. } => {
                DiagnosticBuilder::error("Call of non-function value")
                    .code(ErrorCode::N3004)
                    .primary_label(diag_span, "not a function")
                    .build()
            }
            TypeError::ArgumentCount {
                expected, found, ..
            } => DiagnosticBuilder::error(format!(
                "Argument count mismatch: expected {}, found {}",
                expected, found
            ))
            .code(ErrorCode::N3005)
            .primary_label(
                diag_span,
                format!("expected {} arguments, found {}", expected, found),
            )
            .build(),
            TypeError::MissingMethod {
                interface, method, ..
            } => DiagnosticBuilder::error(format!(
                "Interface `{}` requires method `{}` but the target does not provide it",
                interface, method
            ))
            .code(ErrorCode::N3006)
            .primary_label(
                diag_span,
                format!("missing method `{}` required by `{}`", method, interface),
            )
            .build(),
            TypeError::RecursiveType { .. } => {
                DiagnosticBuilder::error("Occurs check failed: recursive type without indirection")
                    .code(ErrorCode::N3007)
                    .primary_label(diag_span, "recursive type constraint")
                    .build()
            }
            TypeError::Internal { msg, .. } => {
                DiagnosticBuilder::bug(format!("Internal typechecker error: {}", msg))
                    .primary_label(diag_span, "internal typechecker error")
                    .build()
            }
        }
    }
}
