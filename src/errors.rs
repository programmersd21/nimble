use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::lexer::{Span, TokenKind};

// ── Lexer Errors ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Error, Diagnostic)]
pub enum LexError {
    #[error("Illegal tab character")]
    #[diagnostic(code("N0001"))]
    IllegalTab {
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("tabs are not allowed")]
        span: SourceSpan,
        #[help("replace this tab with spaces")]
        help: &'static str,
    },

    #[error("Unexpected character `{ch}`")]
    #[diagnostic(code("N0002"))]
    UnexpectedCharacter {
        ch: char,
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("unexpected character `{ch}`")]
        span: SourceSpan,
    },

    #[error("Unmatched closing delimiter")]
    #[diagnostic(code("N0003"))]
    UnmatchedDelimiter {
        delimiter: char,
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("no matching opening delimiter")]
        span: SourceSpan,
        #[help("remove this extra closing delimiter, or add an opening one")]
        help: &'static str,
    },

    #[error("Invalid float literal `{literal}`")]
    #[diagnostic(code("N0004"))]
    InvalidFloat {
        literal: String,
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("invalid float")]
        span: SourceSpan,
    },

    #[error("Integer literal `{literal}` out of range")]
    #[diagnostic(code("N0005"))]
    IntOverflow {
        literal: String,
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("integer value too large")]
        span: SourceSpan,
        #[help("use a smaller integer literal, or switch to `Float`")]
        help: &'static str,
    },

    #[error("Unterminated string literal")]
    #[diagnostic(code("N0006"))]
    UnterminatedString {
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("missing closing `\"`")]
        span: SourceSpan,
        #[help("add a closing double-quote to terminate this string")]
        help: &'static str,
    },

    #[error("Invalid escape sequence `\\{escape}`")]
    #[diagnostic(code("N0007"))]
    InvalidEscape {
        escape: char,
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("invalid escape")]
        span: SourceSpan,
        #[help(r#"valid escapes: \n, \t, \r, \0, \\, \", \'"#)]
        help: &'static str,
    },

    #[error("Newline inside string literal")]
    #[diagnostic(code("N0008"))]
    NewlineInString {
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("newline inside string")]
        span: SourceSpan,
        #[help("use a multi-line string or escape the newline with \\n")]
        help: &'static str,
    },

    #[error(
        "Indentation error at line {line}: indent level {indent} does not match any enclosing block"
    )]
    #[diagnostic(code("N0009"))]
    IndentationError {
        indent: usize,
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("indent level {indent} is inconsistent")]
        span: SourceSpan,
        #[help("all indentation must align with an enclosing block's indent level")]
        help: &'static str,
    },
}

impl LexError {
    pub fn span(&self) -> Span {
        let (src_span, line, column): (SourceSpan, usize, usize) = match self {
            Self::IllegalTab {
                span, line, column, ..
            } => (*span, *line, *column),
            Self::UnexpectedCharacter {
                span, line, column, ..
            } => (*span, *line, *column),
            Self::UnmatchedDelimiter {
                span, line, column, ..
            } => (*span, *line, *column),
            Self::InvalidFloat {
                span, line, column, ..
            } => (*span, *line, *column),
            Self::IntOverflow {
                span, line, column, ..
            } => (*span, *line, *column),
            Self::UnterminatedString {
                span, line, column, ..
            } => (*span, *line, *column),
            Self::InvalidEscape {
                span, line, column, ..
            } => (*span, *line, *column),
            Self::NewlineInString {
                span, line, column, ..
            } => (*span, *line, *column),
            Self::IndentationError {
                span, line, column, ..
            } => (*span, *line, *column),
        };
        Span::new_with_len(line, column, src_span.offset(), src_span.len())
    }
}

// ── Parser Errors ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Error, Diagnostic)]
pub enum ParseError {
    // Token-level errors
    #[error("Expected `{expected}` but found `{found}` at line {line}:{column}")]
    #[diagnostic(code("N1001"))]
    ExpectedToken {
        expected: String,
        found: String,
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("expected `{expected}`")]
        span: SourceSpan,
    },

    #[error("Unexpected token `{found}` at line {line}:{column}")]
    #[diagnostic(code("N1002"))]
    UnexpectedToken {
        found: String,
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("unexpected `{found}`")]
        span: SourceSpan,
    },

    // Expression errors
    #[error("Expected expression at line {line}:{column}")]
    #[diagnostic(code("N1003"))]
    ExpectedExpression {
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("expected expression")]
        span: SourceSpan,
    },

    #[error("Unclosed `(` at line {line}:{column}")]
    #[diagnostic(code("N1004"))]
    UnclosedParen {
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("unclosed `(`")]
        span: SourceSpan,
    },

    // Block / indentation errors
    #[error("Expected indented block after `:` at line {line}:{column}")]
    #[diagnostic(code("N1005"))]
    ExpectedIndentedBlock {
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("expected indented block")]
        span: SourceSpan,
    },

    #[error("Unexpected indentation at line {line}:{column}")]
    #[diagnostic(code("N1006"))]
    UnexpectedIndent {
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("unexpected indentation")]
        span: SourceSpan,
    },

    // Declaration errors
    #[error("Expected identifier at line {line}:{column}")]
    #[diagnostic(code("N1007"))]
    ExpectedIdentifier {
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("expected identifier")]
        span: SourceSpan,
    },

    #[error("Expected type name at line {line}:{column}")]
    #[diagnostic(code("N1008"))]
    ExpectedType {
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("expected type name")]
        span: SourceSpan,
    },

    #[error("Expected parameter name at line {line}:{column}")]
    #[diagnostic(code("N1009"))]
    ExpectedParameter {
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("expected parameter name")]
        span: SourceSpan,
    },

    // Lex errors
    #[error(transparent)]
    #[diagnostic(transparent)]
    Lex { err: LexError },

    // Internal errors
    #[error("Internal error: {msg}")]
    #[diagnostic(code("N9001"))]
    Internal {
        msg: String,
        #[source_code]
        src: String,
        #[label("internal error")]
        span: SourceSpan,
    },
}

impl ParseError {
    pub(crate) fn expected_token(source: &str, token: &Token, expected: &str) -> Self {
        ParseError::ExpectedToken {
            expected: expected.to_string(),
            found: format_token_kind(&token.kind),
            line: token.span.line,
            column: token.span.column,
            src: source.to_string(),
            span: (token.span.byte_index, token.span.length.max(1)).into(),
        }
    }

    pub(crate) fn unexpected_token(source: &str, token: &Token) -> Self {
        ParseError::UnexpectedToken {
            found: format_token_kind(&token.kind),
            line: token.span.line,
            column: token.span.column,
            src: source.to_string(),
            span: (token.span.byte_index, token.span.length.max(1)).into(),
        }
    }

    pub(crate) fn expected_expression(source: &str, token: &Token) -> Self {
        ParseError::ExpectedExpression {
            line: token.span.line,
            column: token.span.column,
            src: source.to_string(),
            span: (token.span.byte_index, token.span.length.max(1)).into(),
        }
    }

    pub(crate) fn expected_identifier(source: &str, token: &Token) -> Self {
        ParseError::ExpectedIdentifier {
            line: token.span.line,
            column: token.span.column,
            src: source.to_string(),
            span: (token.span.byte_index, token.span.length.max(1)).into(),
        }
    }

    pub(crate) fn expected_indented_block(source: &str, token: &Token) -> Self {
        ParseError::ExpectedIndentedBlock {
            line: token.span.line,
            column: token.span.column,
            src: source.to_string(),
            span: (token.span.byte_index, token.span.length.max(1)).into(),
        }
    }

    /// Reconstruct from `miette::SourceSpan` for LSP error mapping.
    pub fn span(&self) -> crate::lexer::Span {
        let span: SourceSpan = match self {
            ParseError::ExpectedToken { span, .. } => *span,
            ParseError::UnexpectedToken { span, .. } => *span,
            ParseError::ExpectedExpression { span, .. } => *span,
            ParseError::UnclosedParen { span, .. } => *span,
            ParseError::ExpectedIndentedBlock { span, .. } => *span,
            ParseError::UnexpectedIndent { span, .. } => *span,
            ParseError::ExpectedIdentifier { span, .. } => *span,
            ParseError::ExpectedType { span, .. } => *span,
            ParseError::ExpectedParameter { span, .. } => *span,
            ParseError::Lex { .. } => (0usize, 0usize).into(),
            ParseError::Internal { span, .. } => *span,
        };
        crate::lexer::Span::new_with_len(0, 0, span.offset(), span.len())
    }
}

pub fn format_token_kind(kind: &TokenKind) -> String {
    match kind {
        TokenKind::Indent => "INDENT".into(),
        TokenKind::Dedent => "DEDENT".into(),
        TokenKind::Newline => "NEWLINE".into(),
        TokenKind::Colon => "':'".into(),
        TokenKind::ColonEqual => "':='".into(),
        TokenKind::Arrow => "'->'".into(),
        TokenKind::Equal => "'='".into(),
        TokenKind::LParen => "'('".into(),
        TokenKind::RParen => "')'".into(),
        TokenKind::LBracket => "'['".into(),
        TokenKind::RBracket => "']'".into(),
        TokenKind::LBrace => "'{'".into(),
        TokenKind::RBrace => "'}'".into(),
        TokenKind::Fn => "'fn'".into(),
        TokenKind::Let => "'let'".into(),
        TokenKind::Var => "'var'".into(),
        TokenKind::If => "'if'".into(),
        TokenKind::Elif => "'elif'".into(),
        TokenKind::Else => "'else'".into(),
        TokenKind::Struct => "'struct'".into(),
        TokenKind::Interface => "'interface'".into(),
        TokenKind::Pub => "'pub'".into(),
        TokenKind::Return => "'return'".into(),
        TokenKind::While => "'while'".into(),
        TokenKind::Break => "'break'".into(),
        TokenKind::Continue => "'continue'".into(),
        TokenKind::For => "'for'".into(),
        TokenKind::In => "'in'".into(),
        TokenKind::Extern => "'extern'".into(),
        TokenKind::True => "'true'".into(),
        TokenKind::False => "'false'".into(),
        TokenKind::Identifier(s) => format!("identifier `{}`", s),
        TokenKind::IntLiteral(n) => format!("integer `{}`", n),
        TokenKind::FloatLiteral(f) => format!("float `{}`", f),
        TokenKind::StringLiteral(s) => format!("string `{}`", s),
        TokenKind::Plus => "'+'".into(),
        TokenKind::Minus => "'-'".into(),
        TokenKind::Star => "'*'".into(),
        TokenKind::Slash => "'/'".into(),
        TokenKind::Comma => "','".into(),
        TokenKind::Dot => "'.'".into(),
        TokenKind::Bang => "'!'".into(),
        TokenKind::Ampersand => "'&'".into(),
        TokenKind::Pipe => "'|'".into(),
        TokenKind::Greater => "'>'".into(),
        TokenKind::Less => "'<'".into(),
        TokenKind::GreaterEqual => "'>='".into(),
        TokenKind::LessEqual => "'<='".into(),
        TokenKind::EqualEqual => "'=='".into(),
        TokenKind::NotEqual => "'!='".into(),
        TokenKind::PlusEqual => "'+='".into(),
        TokenKind::MinusEqual => "'-='".into(),
        TokenKind::StarEqual => "'*='".into(),
        TokenKind::SlashEqual => "'/='".into(),
        TokenKind::Percent => "'%'".into(),
        TokenKind::PercentEqual => "'%='".into(),
        TokenKind::AmpersandAmpersand => "'&&'".into(),
        TokenKind::PipePipe => "'||'".into(),
        TokenKind::DoubleColon => "'::'".into(),
        TokenKind::Load => "'load'".into(),
        TokenKind::As => "'as'".into(),
        TokenKind::Match => "'match'".into(),
        TokenKind::Enum => "'enum'".into(),
        TokenKind::Mut => "'mut'".into(),
        TokenKind::Defer => "'defer'".into(),
        TokenKind::Macro => "'macro'".into(),
        TokenKind::Async => "'async'".into(),
        TokenKind::Await => "'await'".into(),
        TokenKind::Question => "'?'".into(),
        TokenKind::Eof => "end of file".into(),
    }
}

// ── Name Resolution Errors ────────────────────────────────────────────────

#[derive(Debug, Clone, Error, Diagnostic)]
pub enum ResolveError {
    #[error("Undefined variable `{name}`")]
    #[diagnostic(code("N2001"))]
    UndefinedVariable {
        name: String,
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("`{name}` is not defined in this scope")]
        span: SourceSpan,
        suggestion: Option<String>,
    },

    #[error("Duplicate definition of `{name}`")]
    #[diagnostic(code("N2002"))]
    DuplicateDefinition {
        name: String,
        existing_span: Span,
        new_span: Span,
        #[source_code]
        src: String,
        #[label("`{name}` is already defined")]
        span: SourceSpan,
    },
}

impl ResolveError {
    pub fn span(&self) -> Span {
        match self {
            Self::UndefinedVariable {
                span, line, column, ..
            } => Span::new_with_len(*line, *column, span.offset(), span.len()),
            Self::DuplicateDefinition { span, .. } => Span::new(0, 0, span.offset()),
        }
    }
}

pub use crate::lexer::Token;
