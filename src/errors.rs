use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::lexer::TokenKind;

#[derive(Debug, Error, Diagnostic)]
pub enum ParseError {
    // Token-level errors
    #[error("Expected `{expected}` but found `{found}` at line {line}:{column}")]
    #[diagnostic(code("nimble::parse::expected_token"))]
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
    #[diagnostic(code("nimble::parse::unexpected_token"))]
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
    #[diagnostic(code("nimble::parse::expected_expression"))]
    ExpectedExpression {
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("expected expression")]
        span: SourceSpan,
    },

    #[error("Unclosed `(` at line {line}:{column}")]
    #[diagnostic(code("nimble::parse::unclosed_paren"))]
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
    #[diagnostic(code("nimble::parse::expected_indented_block"))]
    ExpectedIndentedBlock {
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("expected indented block")]
        span: SourceSpan,
    },

    #[error("Unexpected indentation at line {line}:{column}")]
    #[diagnostic(code("nimble::parse::unexpected_indent"))]
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
    #[diagnostic(code("nimble::parse::expected_identifier"))]
    ExpectedIdentifier {
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("expected identifier")]
        span: SourceSpan,
    },

    #[error("Expected type name at line {line}:{column}")]
    #[diagnostic(code("nimble::parse::expected_type"))]
    ExpectedType {
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("expected type name")]
        span: SourceSpan,
    },

    #[error("Expected parameter name at line {line}:{column}")]
    #[diagnostic(code("nimble::parse::expected_parameter"))]
    ExpectedParameter {
        line: usize,
        column: usize,
        #[source_code]
        src: String,
        #[label("expected parameter name")]
        span: SourceSpan,
    },

    // Internal errors
    #[error("Internal error: {msg}")]
    #[diagnostic(code("nimble::parse::internal"))]
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

pub use crate::lexer::Token;
