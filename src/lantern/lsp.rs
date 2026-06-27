// lantern - Backend: LanguageServer trait implementation

use std::sync::Arc;

use dashmap::DashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::env::SymbolKind;
use crate::errors::ParseError;
use crate::lexer::Span;
use crate::lexer::TokenKind;
use crate::typechecker::TypeError;
use crate::{Parser, TypeChecker};

/// Convert a `nimble::Span` into an LSP `Range`.
fn span_to_range(span: &Span, line_index: &[usize]) -> Range {
    let start_line = span.line.saturating_sub(1) as u32;
    let end_line = span.line.saturating_sub(1) as u32;

    let line_start = *line_index.get(span.line.saturating_sub(1)).unwrap_or(&0);
    let start_col = span.byte_index.saturating_sub(line_start) as u32;
    let end_col = start_col + span.length.saturating_sub(1) as u32;

    Range {
        start: Position {
            line: start_line,
            character: start_col,
        },
        end: Position {
            line: end_line,
            character: end_col,
        },
    }
}

/// Build a line-start byte index table for a source string.
fn build_line_index(source: &str) -> Vec<usize> {
    let mut offsets = vec![0usize];
    for (i, ch) in source.char_indices() {
        if ch == '\n' {
            offsets.push(i + 1);
        }
    }
    offsets
}

/// Convert a `ParseError` into LSP `Diagnostic`s.
fn parse_error_to_diagnostics(err: &ParseError, line_index: &[usize]) -> Vec<Diagnostic> {
    let span = err.span();
    let range = span_to_range(&span, line_index);
    vec![Diagnostic {
        range,
        severity: Some(DiagnosticSeverity::ERROR),
        source: Some("nimble".to_string()),
        message: err.to_string(),
        ..Default::default()
    }]
}

/// Convert a `TypeError` into LSP `Diagnostic`s.
fn type_error_to_diagnostics(err: &TypeError, line_index: &[usize]) -> Vec<Diagnostic> {
    let span = err.span();
    let range = span_to_range(&span, line_index);
    vec![Diagnostic {
        range,
        severity: Some(DiagnosticSeverity::ERROR),
        source: Some("nimble".to_string()),
        message: err.to_string(),
        ..Default::default()
    }]
}

/// The LSP backend that tracks open documents and provides diagnostics.
pub struct Backend {
    client: Client,
    /// In-memory document store: URL → source text.
    documents: Arc<DashMap<Url, String>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Backend {
            client,
            documents: Arc::new(DashMap::new()),
        }
    }

    /// Run the full parse + type-check pipeline for a source file and emit
    /// diagnostics back to the client.
    async fn update_diagnostics(&self, uri: &Url) {
        let source = match self.documents.get(uri) {
            Some(s) => s.clone(),
            None => return,
        };
        let line_index = build_line_index(&source);
        let mut diagnostics = Vec::new();

        // Phase 1–2: Lex & Parse
        let prog = match Parser::new(&source) {
            Ok(mut parser) => match parser.parse() {
                Ok(p) => p,
                Err(err) => {
                    diagnostics.extend(parse_error_to_diagnostics(&err, &line_index));
                    let _ = self
                        .client
                        .publish_diagnostics(uri.clone(), diagnostics, None)
                        .await;
                    return;
                }
            },
            Err(err) => {
                diagnostics.extend(parse_error_to_diagnostics(&err, &line_index));
                let _ = self
                    .client
                    .publish_diagnostics(uri.clone(), diagnostics, None)
                    .await;
                return;
            }
        };

        // Phase 3: Type-check
        if let Err(err) = TypeChecker::new(&source).check_program(&prog) {
            diagnostics.extend(type_error_to_diagnostics(&err, &line_index));
        }

        let _ = self
            .client
            .publish_diagnostics(uri.clone(), diagnostics, None)
            .await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "lantern".to_string(),
                version: Some("0.2.0".to_string()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: None,
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
        })
    }

    async fn initialized(&self, _params: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "lantern: initialised")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    // ── Text document synchronisation ─────────────────────────────────

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        self.documents
            .insert(uri.clone(), params.text_document.text);
        self.update_diagnostics(&uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(mut entry) = self.documents.get_mut(&uri) {
            for change in &params.content_changes {
                *entry = change.text.clone();
            }
        }
        self.update_diagnostics(&uri).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents.remove(&params.text_document.uri);
    }

    // ── Hover ─────────────────────────────────────────────────────────

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let source = match self.documents.get(uri) {
            Some(s) => s.clone(),
            None => return Ok(None),
        };
        let pos = params.text_document_position_params.position;

        // Parse and type-check to get the semantic environment.
        let prog = match Parser::new(&source) {
            Ok(mut p) => match p.parse() {
                Ok(p) => p,
                _ => return Ok(None),
            },
            _ => return Ok(None),
        };
        let env = match TypeChecker::new(&source).check_program(&prog) {
            Ok(e) => e,
            _ => return Ok(None),
        };

        // Determine the token at the cursor by scanning the source.
        let line_index = build_line_index(&source);
        let byte_offset =
            line_index.get(pos.line as usize).copied().unwrap_or(0) + pos.character as usize;

        // Find the identifier token at this byte offset.
        let token_at_cursor = {
            let lexer = crate::Lexer::new(&source);
            let mut candidate = None;
            for tok in lexer {
                match tok {
                    Ok(t) => {
                        if t.span.byte_index <= byte_offset
                            && byte_offset < t.span.byte_index + t.span.length.max(1)
                        {
                            candidate = Some(t);
                        }
                    }
                    Err(_) => continue,
                }
            }
            candidate
        };

        if let Some(tok) = token_at_cursor
            && let TokenKind::Identifier(name) = &tok.kind
            && let Some(sym) = env.lookup(name)
        {
            let hover_text = format!(
                "**{}** `{}`  \n---\nkind: {:?}  \nmutable: {}",
                name, sym.type_, sym.kind, sym.mutable
            );
            let range = span_to_range(&tok.span, &line_index);
            return Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: hover_text,
                }),
                range: Some(range),
            }));
        }

        Ok(None)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        if let Some(source) = self.documents.get(&uri) {
            let line = pos.line as usize;
            let col = pos.character as usize;
            let lines: Vec<&str> = source.lines().collect();
            if line < lines.len() && col < lines[line].len() {
                let word = extract_word_at(lines[line], col);
                if let Ok(prog) = Parser::new(&source).and_then(|mut p| p.parse())
                    && let Ok(env) = TypeChecker::new(&source).check_program(&prog)
                    && let Some(sym) = env.lookup(&word)
                {
                    let def_pos = Position::new(
                        (sym.defined_at.line - 1) as u32,
                        (sym.defined_at.column - 1) as u32,
                    );
                    return Ok(Some(GotoDefinitionResponse::Scalar(Location::new(
                        uri.clone(),
                        Range::new(def_pos, def_pos),
                    ))));
                }
            }
        }
        Ok(None)
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let mut items = Vec::new();
        for kw in &[
            "fn",
            "let",
            "var",
            "if",
            "elif",
            "else",
            "match",
            "return",
            "while",
            "for",
            "break",
            "continue",
            "struct",
            "enum",
            "interface",
            "extern",
            "load",
            "pub",
            "as",
            "true",
            "false",
            "defer",
            "mut",
        ] {
            items.push(CompletionItem {
                label: kw.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..Default::default()
            });
        }
        let uri = &params.text_document_position.text_document.uri;
        if let Some(source) = self.documents.get(uri)
            && let Ok(prog) = Parser::new(&source).and_then(|mut p| p.parse())
            && let Ok(env) = TypeChecker::new(&source).check_program(&prog)
        {
            if let Ok(globals) = env.get_globals() {
                for (name, sym) in globals {
                    let kind = match sym.kind {
                        SymbolKind::Function => CompletionItemKind::FUNCTION,
                        SymbolKind::Variable => CompletionItemKind::VARIABLE,
                        SymbolKind::Struct => CompletionItemKind::STRUCT,
                        SymbolKind::Interface => CompletionItemKind::INTERFACE,
                    };
                    items.push(CompletionItem {
                        label: name.clone(),
                        kind: Some(kind),
                        detail: Some(sym.type_.to_string()),
                        ..Default::default()
                    });
                }
            }
        }
        Ok(Some(CompletionResponse::Array(items)))
    }
}

fn extract_word_at(line: &str, col: usize) -> String {
    let bytes = line.as_bytes();
    let mut start = col;
    let mut end = col;
    while start > 0 && ((bytes[start - 1] as char).is_alphanumeric() || bytes[start - 1] == b'_') {
        start -= 1;
    }
    while end < bytes.len() && ((bytes[end] as char).is_alphanumeric() || bytes[end] == b'_') {
        end += 1;
    }
    line[start..end].to_string()
}
