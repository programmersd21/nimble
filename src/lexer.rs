use std::collections::VecDeque;

use crate::errors::LexError;

/// `line`/`column` are 1‑based; `byte_index` is a 0‑based byte offset; `length` is the byte length (0 for virtual tokens).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub byte_index: usize,
    pub length: usize,
}

impl Span {
    pub const fn new(line: usize, column: usize, byte_index: usize) -> Self {
        Span {
            line,
            column,
            byte_index,
            length: 0,
        }
    }

    pub const fn new_with_len(
        line: usize,
        column: usize,
        byte_index: usize,
        length: usize,
    ) -> Self {
        Span {
            line,
            column,
            byte_index,
            length,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TokenKind {
    Indent,
    Dedent,
    Newline,
    Colon,
    DoubleColon,
    Arrow,
    Equal,

    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    ColonEqual,

    Fn,
    Let,
    Var,
    If,
    Elif,
    Else,
    Struct,
    Interface,
    Pub,
    Return,
    While,
    Break,
    Continue,
    For,
    In,
    Extern,
    Load,
    As,
    Match,
    Enum,
    True,
    False,
    Defer,
    Macro,

    Identifier(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),

    Mut,
    Async,
    Await,

    Plus,
    Minus,
    Star,
    Slash,
    Comma,
    Dot,
    Bang,
    Ampersand,
    Pipe,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    EqualEqual,
    NotEqual,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    Percent,
    PercentEqual,
    AmpersandAmpersand,
    PipePipe,
    Question,

    Eof,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub const fn new(kind: TokenKind, span: Span) -> Self {
        Token { kind, span }
    }
}

fn count_leading_whitespace(s: &str) -> usize {
    s.bytes().take_while(|&b| b == b' ').count()
}

fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_' || unicode_xid::UnicodeXID::is_xid_start(ch)
}

fn is_ident_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || unicode_xid::UnicodeXID::is_xid_continue(ch)
}

fn try_multi_char_op(source: &str, pos: usize) -> Option<(usize, TokenKind)> {
    let bytes = source.as_bytes();
    if pos + 1 >= bytes.len() {
        return None;
    }
    let (c0, c1) = (bytes[pos], bytes[pos + 1]);
    let kind = match (c0, c1) {
        (b'-', b'>') => TokenKind::Arrow,
        (b'=', b'=') => TokenKind::EqualEqual,
        (b'!', b'=') => TokenKind::NotEqual,
        (b'>', b'=') => TokenKind::GreaterEqual,
        (b'<', b'=') => TokenKind::LessEqual,
        (b'+', b'=') => TokenKind::PlusEqual,
        (b'-', b'=') => TokenKind::MinusEqual,
        (b'*', b'=') => TokenKind::StarEqual,
        (b'/', b'=') => TokenKind::SlashEqual,
        (b'%', b'=') => TokenKind::PercentEqual,
        (b'&', b'&') => TokenKind::AmpersandAmpersand,
        (b'|', b'|') => TokenKind::PipePipe,
        (b':', b':') => TokenKind::DoubleColon,
        (b':', b'=') => TokenKind::ColonEqual,
        _ => return None,
    };
    Some((2, kind))
}

/// Python-style indentation-aware tokeniser with delimiter tracking.
///
/// Collects non-fatal errors into an internal buffer for later retrieval.
/// The lexer never panics — all invalid input produces structured `LexError`
/// values via the `Result` return, or is collected as a recovered warning.
pub struct Lexer<'a> {
    source: &'a str,
    lines: Vec<&'a str>,
    line_starts: Vec<usize>,

    line_idx: usize,
    pos: usize,

    line_num: usize,
    col: usize,

    indent_stack: Vec<usize>,

    /// Unclosed `(`, `[`, `{` count; >0 suppresses layout tokens.
    delimiter_count: usize,

    pending: VecDeque<Token>,

    eof_emitted: bool,

    line_active: bool,

    /// Non-fatal errors collected during lexing (error recovery).
    errors: Vec<LexError>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let (lines, line_starts) = build_line_table(source);

        let mut lex = Lexer {
            source,
            lines,
            line_starts,
            line_idx: 0,
            pos: 0,
            line_num: 1,
            col: 1,
            indent_stack: vec![0],
            delimiter_count: 0,
            pending: VecDeque::new(),
            eof_emitted: false,
            line_active: false,
            errors: Vec::new(),
        };

        if lex.line_idx < lex.lines.len() {
            lex.skip_to_meaningful_line();
            if lex.line_idx < lex.lines.len() {
                let indent = count_leading_whitespace(lex.current_line());
                if indent > 0 {
                    lex.indent_stack.push(indent);
                    let span = Span::new(1, 1, 0);
                    lex.pending.push_back(Token::new(TokenKind::Indent, span));
                }
                lex.pos = indent;
                lex.col = indent + 1;
            }
        }

        lex
    }

    /// Returns the non-fatal errors accumulated during lexing and clears the buffer.
    pub fn drain_errors(&mut self) -> Vec<LexError> {
        std::mem::take(&mut self.errors)
    }

    pub fn next_token(&mut self) -> Result<Token, LexError> {
        loop {
            if let Some(tok) = self.pending.pop_front() {
                return Ok(tok);
            }

            if self.eof_emitted {
                return Ok(Token::new(
                    TokenKind::Eof,
                    Span::new(self.line_num, self.col, self.source.len()),
                ));
            }

            if self.needs_next_line() {
                let has_next = self.line_idx + 1 < self.lines.len();

                if self.line_active && self.delimiter_count == 0 && has_next {
                    self.line_active = false;
                    return Ok(Token::new(TokenKind::Newline, self.newline_span()));
                }
                self.line_active = false;

                if has_next {
                    self.pos = 0;
                    self.col = 1;

                    if self.advance_to_meaningful_line() && self.delimiter_count == 0 {
                        self.handle_indentation()?;
                    }

                    continue;
                }

                self.flush_dedents_at_eof()?;
                continue;
            }

            self.skip_inline_whitespace();
            if self.pos >= self.current_line().len() {
                continue;
            }

            self.line_active = true;
            return self.tokenize_next();
        }
    }

    pub fn tokenize_all(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token()?;
            let is_eof = matches!(tok.kind, TokenKind::Eof);
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        Ok(tokens)
    }
}

impl<'a> Lexer<'a> {
    fn current_line(&self) -> &'a str {
        self.lines[self.line_idx]
    }

    fn current_line_start(&self) -> usize {
        self.line_starts[self.line_idx]
    }

    fn needs_next_line(&self) -> bool {
        self.line_idx >= self.lines.len() || self.pos >= self.current_line().len()
    }

    fn span_at_current_pos(&self, len: usize) -> Span {
        Span::new_with_len(
            self.line_num,
            self.col,
            self.current_line_start() + self.pos,
            len,
        )
    }

    fn newline_span(&self) -> Span {
        let line_len = self.current_line().len();
        Span::new_with_len(
            self.line_num,
            line_len + 1,
            self.current_line_start() + line_len,
            1,
        )
    }

    fn skip_to_meaningful_line(&mut self) {
        while self.line_idx < self.lines.len() {
            let trimmed = self.lines[self.line_idx].trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                self.line_idx += 1;
                self.line_num += 1;
            } else {
                break;
            }
        }
    }

    /// Returns `false` when the end of line table reached.
    fn advance_to_meaningful_line(&mut self) -> bool {
        loop {
            let next = self.line_idx + 1;
            if next >= self.lines.len() {
                return false;
            }
            self.line_idx = next;
            self.line_num += 1;
            let trimmed = self.lines[self.line_idx].trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                return true;
            }
        }
    }

    fn flush_dedents_at_eof(&mut self) -> Result<(), LexError> {
        while self.indent_stack.len() > 1 {
            self.indent_stack.pop();
            self.pending.push_back(Token::new(
                TokenKind::Dedent,
                Span::new(self.line_num, self.col, self.source.len()),
            ));
        }
        self.eof_emitted = true;
        Ok(())
    }

    fn handle_indentation(&mut self) -> Result<(), LexError> {
        let raw = self.current_line();
        let indent = count_leading_whitespace(raw);

        let top = *self.indent_stack.last().unwrap();

        if indent > top {
            self.indent_stack.push(indent);
            let span = Span::new(self.line_num, 1, self.current_line_start());
            self.pending.push_back(Token::new(TokenKind::Indent, span));
        } else if indent < top {
            while *self.indent_stack.last().unwrap() > indent {
                self.indent_stack.pop().unwrap();
                let span = Span::new(self.line_num, 1, self.current_line_start());
                self.pending.push_back(Token::new(TokenKind::Dedent, span));
            }
            if self.indent_stack.is_empty() {
                self.indent_stack.push(0);
            }
            if *self.indent_stack.last().unwrap() != indent
                && (indent != 0 || self.indent_stack.len() != 1)
            {
                return Err(LexError::IndentationError {
                    indent,
                    line: self.line_num,
                    column: 1,
                    src: self.source.to_string(),
                    span: (self.current_line_start(), raw.len()).into(),
                    help: "all indentation must align with an enclosing block's indent level",
                });
            }
        }

        self.pos = indent;
        self.col = indent + 1;

        Ok(())
    }

    fn skip_inline_whitespace(&mut self) {
        let bytes = self.current_line().as_bytes();
        while self.pos < bytes.len() {
            match bytes[self.pos] {
                b' ' => {
                    self.pos += 1;
                    self.col += 1;
                }
                b'\t' => return,
                _ => break,
            }
        }
    }

    /// Tokenise the next lexeme at [`self.pos`].
    fn tokenize_next(&mut self) -> Result<Token, LexError> {
        let bytes = self.current_line().as_bytes();
        let ch = bytes[self.pos];

        if let Some((width, kind)) = try_multi_char_op(self.current_line(), self.pos) {
            let span = self.span_at_current_pos(width);
            self.advance_pos(width);
            return Ok(Token::new(kind, span));
        }

        let (width, kind) = match ch {
            b'(' => (1, TokenKind::LParen),
            b')' => (1, TokenKind::RParen),
            b'[' => (1, TokenKind::LBracket),
            b']' => (1, TokenKind::RBracket),
            b'{' => (1, TokenKind::LBrace),
            b'}' => (1, TokenKind::RBrace),
            b':' => (1, TokenKind::Colon),
            b'=' => (1, TokenKind::Equal),
            b'+' => (1, TokenKind::Plus),
            b'-' => (1, TokenKind::Minus),
            b'*' => (1, TokenKind::Star),
            b'/' => (1, TokenKind::Slash),
            b'%' => (1, TokenKind::Percent),
            b',' => (1, TokenKind::Comma),
            b'.' => (1, TokenKind::Dot),
            b'!' => (1, TokenKind::Bang),
            b'&' => (1, TokenKind::Ampersand),
            b'|' => (1, TokenKind::Pipe),
            b'>' => (1, TokenKind::Greater),
            b'<' => (1, TokenKind::Less),
            b'?' => (1, TokenKind::Question),

            b'\t' => {
                return Err(LexError::IllegalTab {
                    line: self.line_num,
                    column: self.col,
                    src: self.source.to_string(),
                    span: (self.current_line_start() + self.pos, 1).into(),
                    help: "replace this tab with spaces",
                });
            }

            b'0'..=b'9' => return self.tokenize_number(),
            b'"' => return self.tokenize_string(),

            _ if is_ident_start(ch as char) => return self.tokenize_identifier_or_keyword(),

            _ => {
                return Err(LexError::UnexpectedCharacter {
                    ch: ch as char,
                    line: self.line_num,
                    column: self.col,
                    src: self.source.to_string(),
                    span: (self.current_line_start() + self.pos, 1).into(),
                });
            }
        };

        let span = self.span_at_current_pos(width);

        match kind {
            TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace => {
                self.delimiter_count += 1
            }
            TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace => {
                if self.delimiter_count == 0 {
                    let delim = match kind {
                        TokenKind::RParen => ')',
                        TokenKind::RBracket => ']',
                        _ => '}',
                    };
                    return Err(LexError::UnmatchedDelimiter {
                        delimiter: delim,
                        line: self.line_num,
                        column: self.col,
                        src: self.source.to_string(),
                        span: (self.current_line_start() + self.pos, 1).into(),
                        help: "remove this extra closing delimiter, or add an opening one",
                    });
                }
                self.delimiter_count -= 1;
            }
            _ => {}
        }

        self.advance_pos(width);
        Ok(Token::new(kind, span))
    }

    fn tokenize_number(&mut self) -> Result<Token, LexError> {
        let start_pos = self.pos;
        let bytes = self.current_line().as_bytes();
        let line_start = self.current_line_start();

        while self.pos < bytes.len() && bytes[self.pos].is_ascii_digit() {
            self.pos += 1;
        }

        if self.pos < bytes.len()
            && bytes[self.pos] == b'.'
            && self.pos + 1 < bytes.len()
            && bytes[self.pos + 1].is_ascii_digit()
        {
            self.pos += 1;
            while self.pos < bytes.len() && bytes[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
            let raw = &self.current_line()[start_pos..self.pos];
            let value: f64 = raw.parse().map_err(|_| LexError::InvalidFloat {
                literal: raw.to_string(),
                line: self.line_num,
                column: start_pos + 1,
                src: self.source.to_string(),
                span: (line_start + start_pos, self.pos - start_pos).into(),
            })?;
            let len = self.pos - start_pos;
            let span =
                Span::new_with_len(self.line_num, start_pos + 1, line_start + start_pos, len);
            self.col += len;
            Ok(Token::new(TokenKind::FloatLiteral(value), span))
        } else {
            let raw = &self.current_line()[start_pos..self.pos];
            let value: i64 = raw.parse().map_err(|_| LexError::IntOverflow {
                literal: raw.to_string(),
                line: self.line_num,
                column: start_pos + 1,
                src: self.source.to_string(),
                span: (line_start + start_pos, self.pos - start_pos).into(),
                help: "use a smaller integer literal, or switch to `Float`",
            })?;
            let len = self.pos - start_pos;
            let span =
                Span::new_with_len(self.line_num, start_pos + 1, line_start + start_pos, len);
            self.col += len;
            Ok(Token::new(TokenKind::IntLiteral(value), span))
        }
    }

    fn tokenize_string(&mut self) -> Result<Token, LexError> {
        let start_pos = self.pos;
        let byte_offset = self.current_line_start() + start_pos;
        self.pos += 1;
        self.col += 1;

        let mut value = String::new();
        let bytes = self.current_line().as_bytes();

        loop {
            if self.pos >= bytes.len() {
                return Err(LexError::UnterminatedString {
                    line: self.line_num,
                    column: start_pos + 1,
                    src: self.source.to_string(),
                    span: (byte_offset, self.pos - start_pos).into(),
                    help: "add a closing double-quote to terminate this string",
                });
            }
            match bytes[self.pos] {
                b'"' => {
                    self.pos += 1;
                    self.col += 1;
                    let len = self.pos - start_pos;
                    let span = Span::new_with_len(self.line_num, start_pos + 1, byte_offset, len);
                    return Ok(Token::new(TokenKind::StringLiteral(value), span));
                }
                b'\\' => {
                    self.pos += 1;
                    self.col += 1;
                    if self.pos >= bytes.len() {
                        return Err(LexError::UnterminatedString {
                            line: self.line_num,
                            column: start_pos + 1,
                            src: self.source.to_string(),
                            span: (byte_offset, self.pos - start_pos).into(),
                            help: "add a closing double-quote to terminate this string",
                        });
                    }
                    let escaped = match bytes[self.pos] {
                        b'n' => '\n',
                        b't' => '\t',
                        b'r' => '\r',
                        b'0' => '\0',
                        b'\\' => '\\',
                        b'"' => '"',
                        b'\'' => '\'',
                        other => {
                            return Err(LexError::InvalidEscape {
                                escape: other as char,
                                line: self.line_num,
                                column: self.col,
                                src: self.source.to_string(),
                                span: (byte_offset + self.pos - start_pos - 1, 2).into(),
                                help: r#"valid escapes: \n, \t, \r, \0, \\, \", \'"#,
                            });
                        }
                    };
                    value.push(escaped);
                    self.pos += 1;
                    self.col += 1;
                }
                b'\n' | b'\r' => {
                    return Err(LexError::NewlineInString {
                        line: self.line_num,
                        column: self.col,
                        src: self.source.to_string(),
                        span: (byte_offset + self.pos - start_pos, 1).into(),
                        help: "use a multi-line string or escape the newline with \\n",
                    });
                }
                _ => {
                    let rest = self.current_line();
                    let rest = &rest[self.pos..];
                    if let Some(ch) = rest.chars().next() {
                        value.push(ch);
                        let byte_len = ch.len_utf8();
                        self.pos += byte_len;
                        self.col += 1;
                    } else {
                        self.pos += 1;
                        self.col += 1;
                    }
                }
            }
        }
    }

    fn tokenize_identifier_or_keyword(&mut self) -> Result<Token, LexError> {
        let start_pos = self.pos;
        let line_start = self.current_line_start();
        let line_str = self.current_line();
        let rest = &line_str[start_pos..];

        // Use UTF-8 character boundaries via char_indices.
        let mut raw_len = 0usize;
        for (i, ch) in rest.char_indices() {
            if i == 0 {
                if !is_ident_start(ch) {
                    // The actual character is not a valid identifier start (even though the
                    // byte-level check in tokenize_next suggested it might be). Emit an error
                    // and advance past this character so we don't get stuck.
                    let char_end = start_pos + ch.len_utf8();
                    self.pos = char_end;
                    self.col += ch.len_utf8();
                    return Err(LexError::UnexpectedCharacter {
                        ch,
                        line: self.line_num,
                        column: start_pos + 1,
                        src: self.source.to_string(),
                        span: (line_start + start_pos, ch.len_utf8()).into(),
                    });
                }
                raw_len = ch.len_utf8();
            } else if is_ident_continue(ch) {
                raw_len = i + ch.len_utf8();
            } else {
                break;
            }
        }

        if raw_len == 0 {
            // Should not be reachable, but guard against infinite loops.
            self.pos = start_pos + 1;
            self.col += 1;
            return Err(LexError::UnexpectedCharacter {
                ch: '\0',
                line: self.line_num,
                column: start_pos + 1,
                src: self.source.to_string(),
                span: (line_start + start_pos, 1).into(),
            });
        }

        self.pos = start_pos + raw_len;
        let raw = &line_str[start_pos..self.pos];
        let len = raw_len;
        let span = Span::new_with_len(self.line_num, start_pos + 1, line_start + start_pos, len);
        self.col += len;

        let kind = match raw {
            "fn" => TokenKind::Fn,
            "let" => TokenKind::Let,
            "var" => TokenKind::Var,
            "if" => TokenKind::If,
            "elif" => TokenKind::Elif,
            "else" => TokenKind::Else,
            "struct" => TokenKind::Struct,
            "interface" => TokenKind::Interface,
            "pub" => TokenKind::Pub,
            "return" => TokenKind::Return,
            "while" => TokenKind::While,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "for" => TokenKind::For,
            "in" => TokenKind::In,
            "extern" => TokenKind::Extern,
            "load" => TokenKind::Load,
            "as" => TokenKind::As,
            "match" => TokenKind::Match,
            "enum" => TokenKind::Enum,
            "mut" => TokenKind::Mut,
            "async" => TokenKind::Async,
            "await" => TokenKind::Await,
            "defer" => TokenKind::Defer,
            "macro" => TokenKind::Macro,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            ident => TokenKind::Identifier(ident.to_string()),
        };

        Ok(Token::new(kind, span))
    }

    fn advance_pos(&mut self, width: usize) {
        self.pos += width;
        self.col += width;
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(tok) if matches!(tok.kind, TokenKind::Eof) => None,
            other => Some(other),
        }
    }
}

fn build_line_table(source: &str) -> (Vec<&str>, Vec<usize>) {
    let src_len = source.len();
    let bytes = source.as_bytes();
    let mut lines = Vec::new();
    let mut starts = Vec::new();
    let mut pos = 0usize;

    while pos < src_len {
        starts.push(pos);
        let line_start = pos;

        while pos < src_len && bytes[pos] != b'\n' && bytes[pos] != b'\r' {
            pos += 1;
        }

        lines.push(&source[line_start..pos]);

        if pos < src_len && bytes[pos] == b'\r' {
            pos += 1;
        }
        if pos < src_len && bytes[pos] == b'\n' {
            pos += 1;
        }
    }

    if !source.is_empty() && (source.ends_with('\n') || source.ends_with('\r')) {
        starts.push(pos);
        lines.push("");
    }

    (lines, starts)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(source: &str) -> Vec<(TokenKind, Span)> {
        let mut lex = Lexer::new(source);
        let mut result = Vec::new();
        loop {
            let tok = lex.next_token().expect("lexer error");
            let is_eof = matches!(tok.kind, TokenKind::Eof);
            result.push((tok.kind, tok.span));
            if is_eof {
                break;
            }
        }
        result
    }

    fn kinds(source: &str) -> Vec<TokenKind> {
        let mut lex = Lexer::new(source);
        let mut result = Vec::new();
        loop {
            let tok = lex.next_token().expect("lexer error");
            let is_eof = matches!(tok.kind, TokenKind::Eof);
            result.push(tok.kind);
            if is_eof {
                break;
            }
        }
        result
    }

    fn kinds_no_eof(source: &str) -> Vec<TokenKind> {
        let mut v = kinds(source);
        v.pop();
        v
    }

    #[test]
    fn lex_errors_are_structured() {
        let mut lex = Lexer::new("\t42");
        let err = lex.next_token().unwrap_err();
        match err {
            LexError::IllegalTab { .. } => {}
            other => panic!("expected IllegalTab, got {:?}", other),
        }
    }

    #[test]
    fn empty_source() {
        let toks = kinds("");
        assert_eq!(toks, vec![TokenKind::Eof]);
    }

    #[test]
    fn load_keyword() {
        let toks = kinds_no_eof("load std.io");
        assert_eq!(
            toks,
            vec![
                TokenKind::Load,
                TokenKind::Identifier("std".into()),
                TokenKind::Dot,
                TokenKind::Identifier("io".into()),
            ],
        );
    }

    #[test]
    fn identifier_then_newline() {
        let toks = kinds_no_eof("hello\nworld");
        assert_eq!(
            toks,
            vec![
                TokenKind::Identifier("hello".into()),
                TokenKind::Newline,
                TokenKind::Identifier("world".into()),
            ],
        );
    }

    #[test]
    fn all_keywords() {
        let toks = kinds_no_eof(
            "fn let var if elif else struct interface pub mut return in extern for while match enum defer macro async await true false",
        );
        assert_eq!(
            toks,
            vec![
                TokenKind::Fn,
                TokenKind::Let,
                TokenKind::Var,
                TokenKind::If,
                TokenKind::Elif,
                TokenKind::Else,
                TokenKind::Struct,
                TokenKind::Interface,
                TokenKind::Pub,
                TokenKind::Mut,
                TokenKind::Return,
                TokenKind::In,
                TokenKind::Extern,
                TokenKind::For,
                TokenKind::While,
                TokenKind::Match,
                TokenKind::Enum,
                TokenKind::Defer,
                TokenKind::Macro,
                TokenKind::Async,
                TokenKind::Await,
                TokenKind::True,
                TokenKind::False,
            ],
        );
    }

    #[test]
    fn delimiters() {
        let toks = kinds_no_eof("( ) [ ] { } : -> = , .");
        assert_eq!(
            toks,
            vec![
                TokenKind::LParen,
                TokenKind::RParen,
                TokenKind::LBracket,
                TokenKind::RBracket,
                TokenKind::LBrace,
                TokenKind::RBrace,
                TokenKind::Colon,
                TokenKind::Arrow,
                TokenKind::Equal,
                TokenKind::Comma,
                TokenKind::Dot,
            ],
        );
    }

    #[test]
    fn operators() {
        let src = "+ - * / ! & | > < == != >= <= += -= *= /= && ||";
        let toks = kinds_no_eof(src);
        assert_eq!(
            toks,
            vec![
                TokenKind::Plus,
                TokenKind::Minus,
                TokenKind::Star,
                TokenKind::Slash,
                TokenKind::Bang,
                TokenKind::Ampersand,
                TokenKind::Pipe,
                TokenKind::Greater,
                TokenKind::Less,
                TokenKind::EqualEqual,
                TokenKind::NotEqual,
                TokenKind::GreaterEqual,
                TokenKind::LessEqual,
                TokenKind::PlusEqual,
                TokenKind::MinusEqual,
                TokenKind::StarEqual,
                TokenKind::SlashEqual,
                TokenKind::AmpersandAmpersand,
                TokenKind::PipePipe,
            ],
        );
    }

    #[test]
    fn integer_literals() {
        let toks = kinds_no_eof("0 42 100");
        assert_eq!(
            toks,
            vec![
                TokenKind::IntLiteral(0),
                TokenKind::IntLiteral(42),
                TokenKind::IntLiteral(100),
            ],
        );
    }

    #[test]
    fn float_literals() {
        let toks = kinds_no_eof("0.0 3.14 2.5");
        assert_eq!(
            toks,
            vec![
                TokenKind::FloatLiteral(0.0),
                TokenKind::FloatLiteral(3.14),
                TokenKind::FloatLiteral(2.5),
            ],
        );
    }

    #[test]
    fn float_must_have_digit_after_dot() {
        let toks = kinds_no_eof("42.");
        assert_eq!(toks, vec![TokenKind::IntLiteral(42), TokenKind::Dot,],);
    }

    #[test]
    fn string_literals() {
        let toks = kinds_no_eof(r#""hello" "world""#);
        assert_eq!(
            toks,
            vec![
                TokenKind::StringLiteral("hello".into()),
                TokenKind::StringLiteral("world".into()),
            ],
        );
    }

    #[test]
    fn string_with_escapes() {
        let toks = kinds_no_eof(r#""a\nb\tc\\d\"""#);
        assert_eq!(toks, vec![TokenKind::StringLiteral("a\nb\tc\\d\"".into())],);
    }

    #[test]
    fn string_with_emoji() {
        let toks = kinds_no_eof(r#""hello 😀 world""#);
        assert_eq!(
            toks,
            vec![TokenKind::StringLiteral("hello 😀 world".into())],
        );
    }

    #[test]
    fn string_with_unicode_chinese() {
        let toks = kinds_no_eof(r#""你好世界""#);
        assert_eq!(toks, vec![TokenKind::StringLiteral("你好世界".into())],);
    }

    #[test]
    fn string_with_mixed_unicode() {
        let toks = kinds_no_eof(r#""café résumé 100% ✓""#);
        assert_eq!(
            toks,
            vec![TokenKind::StringLiteral("café résumé 100% ✓".into())],
        );
    }

    #[test]
    fn unterminated_string_error() {
        let mut lex = Lexer::new(r#""hello"#);
        let err = lex.next_token().unwrap_err();
        match err {
            LexError::UnterminatedString { .. } => {}
            other => panic!("expected UnterminatedString, got {:?}", other),
        }
    }

    #[test]
    fn invalid_escape_error() {
        let mut lex = Lexer::new(r#""hello\z""#);
        let err = lex.next_token().unwrap_err();
        match err {
            LexError::InvalidEscape { escape, .. } => assert_eq!(escape, 'z'),
            other => panic!("expected InvalidEscape, got {:?}", other),
        }
    }

    #[test]
    fn newline_in_string_error() {
        // Newlines are consumed by build_line_table before lexing, so the lexer
        // never sees raw newlines inside the line buffer. An unterminated quote
        // at end-of-line produces UnterminatedString instead.
        let mut lex = Lexer::new("\"hello\nworld\"");
        let err = lex.next_token().unwrap_err();
        match err {
            LexError::UnterminatedString { .. } => {}
            other => panic!("expected UnterminatedString, got {:?}", other),
        }
    }

    #[test]
    fn full_line_comment() {
        let toks = kinds_no_eof("# this is a comment\n42");
        assert_eq!(toks, vec![TokenKind::IntLiteral(42)]);
    }

    #[test]
    fn inline_comment_is_not_special() {
        let mut lex = Lexer::new("42 # not a comment");
        assert!(lex.next_token().is_ok()); // 42
        let err = lex.next_token().unwrap_err();
        match err {
            LexError::UnexpectedCharacter { .. } => {}
            other => panic!("expected UnexpectedCharacter, got {:?}", other),
        }
    }

    #[test]
    fn single_indent_dedent() {
        let src = "a\n    b\nc";
        let toks = kinds_no_eof(src);
        assert_eq!(
            toks,
            vec![
                TokenKind::Identifier("a".into()),
                TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Identifier("b".into()),
                TokenKind::Newline,
                TokenKind::Dedent,
                TokenKind::Identifier("c".into()),
            ],
        );
    }

    #[test]
    fn nested_indent() {
        let src = "a\n    b\n        c\nd";
        let toks = kinds_no_eof(src);
        assert_eq!(
            toks,
            vec![
                TokenKind::Identifier("a".into()),
                TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Identifier("b".into()),
                TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Identifier("c".into()),
                TokenKind::Newline,
                TokenKind::Dedent,
                TokenKind::Dedent,
                TokenKind::Identifier("d".into()),
            ],
        );
    }

    #[test]
    fn multiple_dedents_at_once() {
        let src = "a\n    b\n        c\n    d\ne";
        let toks = kinds_no_eof(src);
        assert_eq!(
            toks,
            vec![
                TokenKind::Identifier("a".into()),
                TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Identifier("b".into()),
                TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Identifier("c".into()),
                TokenKind::Newline,
                TokenKind::Dedent,
                TokenKind::Identifier("d".into()),
                TokenKind::Newline,
                TokenKind::Dedent,
                TokenKind::Identifier("e".into()),
            ],
        );
    }

    #[test]
    fn indentation_error_on_dedent_mismatch() {
        let src = "a\n    b\n  c";
        let mut lex = Lexer::new(src);
        assert!(lex.next_token().is_ok()); // a
        assert!(lex.next_token().is_ok()); // Newline
        assert!(lex.next_token().is_ok()); // Indent
        assert!(lex.next_token().is_ok()); // b
        assert!(lex.next_token().is_ok()); // Newline
        let err = lex.next_token().unwrap_err();
        match err {
            LexError::IndentationError { .. } => {}
            other => panic!("expected IndentationError, got {:?}", other),
        }
    }

    #[test]
    fn blank_lines_between_code() {
        let src = "a\n\n\nb";
        let toks = kinds_no_eof(src);
        assert_eq!(
            toks,
            vec![
                TokenKind::Identifier("a".into()),
                TokenKind::Newline,
                TokenKind::Identifier("b".into()),
            ],
        );
    }

    #[test]
    fn blank_lines_with_comments() {
        let src = "a\n# comment\n\nb";
        let toks = kinds_no_eof(src);
        assert_eq!(
            toks,
            vec![
                TokenKind::Identifier("a".into()),
                TokenKind::Newline,
                TokenKind::Identifier("b".into()),
            ],
        );
    }

    #[test]
    fn parens_suppress_newline() {
        let src = "(\n    42\n)";
        let toks = kinds_no_eof(src);
        assert_eq!(
            toks,
            vec![
                TokenKind::LParen,
                TokenKind::IntLiteral(42),
                TokenKind::RParen,
            ],
        );
    }

    #[test]
    fn nested_brackets_suppress_layout() {
        let src = "a(\n    b(\n        c\n    )\n)";
        let toks = kinds_no_eof(src);
        assert_eq!(
            toks,
            vec![
                TokenKind::Identifier("a".into()),
                TokenKind::LParen,
                TokenKind::Identifier("b".into()),
                TokenKind::LParen,
                TokenKind::Identifier("c".into()),
                TokenKind::RParen,
                TokenKind::RParen,
            ],
        );
    }

    #[test]
    fn mixed_brackets() {
        let src = "x = [\n    1,\n    2,\n]";
        let toks = kinds_no_eof(src);
        assert_eq!(
            toks,
            vec![
                TokenKind::Identifier("x".into()),
                TokenKind::Equal,
                TokenKind::LBracket,
                TokenKind::IntLiteral(1),
                TokenKind::Comma,
                TokenKind::IntLiteral(2),
                TokenKind::Comma,
                TokenKind::RBracket,
            ],
        );
    }

    #[test]
    fn indent_inside_continuation_off() {
        let src = "a = [\n    1\n  ]";
        let toks = kinds_no_eof(src);
        assert_eq!(
            toks,
            vec![
                TokenKind::Identifier("a".into()),
                TokenKind::Equal,
                TokenKind::LBracket,
                TokenKind::IntLiteral(1),
                TokenKind::RBracket,
            ],
        );
    }

    #[test]
    fn unmatched_closing_delimiter_error() {
        let mut lex = Lexer::new(")");
        let err = lex.next_token().unwrap_err();
        match err {
            LexError::UnmatchedDelimiter { delimiter, .. } => assert_eq!(delimiter, ')'),
            other => panic!("expected UnmatchedDelimiter, got {:?}", other),
        }
    }

    #[test]
    fn full_function_definition() {
        let src = "fn greet(name):\n    return \"Hello, \" + name\n";
        let toks = kinds_no_eof(src);
        assert_eq!(
            toks,
            vec![
                TokenKind::Fn,
                TokenKind::Identifier("greet".into()),
                TokenKind::LParen,
                TokenKind::Identifier("name".into()),
                TokenKind::RParen,
                TokenKind::Colon,
                TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Return,
                TokenKind::StringLiteral("Hello, ".into()),
                TokenKind::Plus,
                TokenKind::Identifier("name".into()),
                TokenKind::Newline,
                TokenKind::Dedent,
            ],
        );
    }

    #[test]
    fn if_elif_else() {
        let src = "if a:\n    x\nelif b:\n    y\nelse:\n    z\n";
        let toks = kinds_no_eof(src);
        assert_eq!(
            toks,
            vec![
                TokenKind::If,
                TokenKind::Identifier("a".into()),
                TokenKind::Colon,
                TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Identifier("x".into()),
                TokenKind::Newline,
                TokenKind::Dedent,
                TokenKind::Elif,
                TokenKind::Identifier("b".into()),
                TokenKind::Colon,
                TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Identifier("y".into()),
                TokenKind::Newline,
                TokenKind::Dedent,
                TokenKind::Else,
                TokenKind::Colon,
                TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Identifier("z".into()),
                TokenKind::Newline,
                TokenKind::Dedent,
            ],
        );
    }

    #[test]
    fn span_positions_single_line() {
        let toks = lex("fn foo(x)");
        // fn  (len 2)
        assert_eq!(toks[0].1, Span::new_with_len(1, 1, 0, 2));
        // foo (len 3)
        assert_eq!(toks[1].1, Span::new_with_len(1, 4, 3, 3));
        // (   (len 1)
        assert_eq!(toks[2].1, Span::new_with_len(1, 7, 6, 1));
        // x   (len 1)
        assert_eq!(toks[3].1, Span::new_with_len(1, 8, 7, 1));
        // )   (len 1)
        assert_eq!(toks[4].1, Span::new_with_len(1, 9, 8, 1));
        // Eof (len 0)
        assert_eq!(toks[5].1, Span::new(1, 10, 9));
    }

    #[test]
    fn span_positions_multi_line() {
        let toks = lex("a\n    b\n");
        // a  (len 1)
        assert_eq!(toks[0].1, Span::new_with_len(1, 1, 0, 1));
        // Newline (len 1)
        assert_eq!(toks[1].1, Span::new_with_len(1, 2, 1, 1));
        // Indent (len 0)
        assert_eq!(toks[2].1, Span::new(2, 1, 2));
        // b  (len 1)
        assert_eq!(toks[3].1, Span::new_with_len(2, 5, 6, 1));
        // Newline (len 1)
        assert_eq!(toks[4].1, Span::new_with_len(2, 6, 7, 1));
        // Dedent (len 0, at EOF on the trailing empty line)
        assert_eq!(toks[5].1, Span::new(3, 1, 8));
        // Eof (len 0)
        assert_eq!(toks[6].1, Span::new(3, 1, 8));
    }

    #[test]
    fn file_ends_without_newline() {
        let src = "hello";
        let toks = kinds_no_eof(src);
        assert_eq!(toks, vec![TokenKind::Identifier("hello".into())]);
    }

    #[test]
    fn file_ends_with_indented_block() {
        let src = "a\n    b";
        let toks = kinds_no_eof(src);
        assert_eq!(
            toks,
            vec![
                TokenKind::Identifier("a".into()),
                TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Identifier("b".into()),
                TokenKind::Dedent,
            ],
        );
    }

    #[test]
    fn carriage_return_line_feeds() {
        let src = "a\r\n    b\r\nc";
        let toks = kinds_no_eof(src);
        assert_eq!(
            toks,
            vec![
                TokenKind::Identifier("a".into()),
                TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Identifier("b".into()),
                TokenKind::Newline,
                TokenKind::Dedent,
                TokenKind::Identifier("c".into()),
            ],
        );
    }

    #[test]
    fn tab_char_error() {
        let mut lex = Lexer::new("\t42");
        let err = lex.next_token().unwrap_err();
        match err {
            LexError::IllegalTab { .. } => {}
            other => panic!("expected IllegalTab, got {:?}", other),
        }
    }

    #[test]
    fn only_comments_and_blanks() {
        let src = "# just a comment\n\n  # indented comment\n";
        let toks = kinds_no_eof(src);
        assert!(toks.is_empty());
    }

    #[test]
    fn tokenize_all_trait_eof_is_none() {
        let mut lex = Lexer::new("42");
        let tokens: Vec<_> = lex.by_ref().collect::<Result<Vec<_>, _>>().unwrap();
        let kinds: Vec<_> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(kinds, vec![TokenKind::IntLiteral(42)]);
    }

    // ── Unicode identifier tests ────────────────────────────────────────────

    #[test]
    fn unicode_identifiers_greek() {
        let toks = kinds_no_eof("α = 1\n");
        assert_eq!(toks[0], TokenKind::Identifier("α".to_string()),);
    }

    #[test]
    fn unicode_identifiers_cyrillic() {
        let toks = kinds_no_eof("привет = 42\n");
        assert_eq!(toks[0], TokenKind::Identifier("привет".to_string()),);
    }

    #[test]
    fn unicode_identifiers_cjk() {
        let toks = kinds_no_eof("变量 = 100\n");
        assert_eq!(toks[0], TokenKind::Identifier("变量".to_string()),);
    }

    #[test]
    fn unicode_identifiers_mixed() {
        let toks = kinds_no_eof("my_αβγ = 3.14\n");
        assert_eq!(toks[0], TokenKind::Identifier("my_αβγ".to_string()),);
    }

    #[test]
    fn unicode_identifiers_with_digits() {
        // '²' (U+00B2 superscript two) is NOT XID_Continue (it is No, not Nd).
        // The identifier is just "x", then the superscript is unexpected.
        let mut lex = Lexer::new("x² = 4\n");
        let first = lex.next_token().unwrap();
        assert_eq!(first.kind, TokenKind::Identifier("x".to_string()));
        let err = lex.next_token().unwrap_err();
        match err {
            LexError::UnexpectedCharacter { ch, .. } => {
                assert_eq!(ch, '\u{00B2}');
            }
            other => panic!("expected UnexpectedCharacter, got {:?}", other),
        }
    }

    #[test]
    fn unicode_identifiers_emoji_not_ident_start() {
        let mut lex = Lexer::new("😀 = 1\n");
        let err = lex.next_token().unwrap_err();
        match err {
            LexError::UnexpectedCharacter { .. } => {}
            other => panic!("expected UnexpectedCharacter, got {:?}", other),
        }
    }

    #[test]
    fn unicode_identifiers_after_keyword() {
        let toks = kinds_no_eof("let π = 3.14\n");
        assert_eq!(
            toks,
            vec![
                TokenKind::Let,
                TokenKind::Identifier("π".to_string()),
                TokenKind::Equal,
                TokenKind::FloatLiteral(3.14),
                TokenKind::Newline,
            ],
        );
    }

    // ── LexError error code tests ──────────────────────────────────────────

    #[test]
    fn lex_error_has_code() {
        let err = LexError::IllegalTab {
            line: 1,
            column: 1,
            src: String::new(),
            span: (0usize, 0usize).into(),
            help: "",
        };
        let report = format!("{:?}", miette::Report::new(err));
        assert!(report.contains("nimble::lex::illegal_tab"));
    }

    #[test]
    fn lex_error_display() {
        let err = LexError::UnterminatedString {
            line: 1,
            column: 5,
            src: String::new(),
            span: (0usize, 0usize).into(),
            help: "",
        };
        let msg = format!("{}", err);
        assert!(msg.contains("Unterminated string literal"));
    }

    #[test]
    fn lex_error_span_roundtrip() {
        let err = LexError::IllegalTab {
            line: 3,
            column: 7,
            src: "test".to_string(),
            span: (10usize, 1usize).into(),
            help: "",
        };
        let span = err.span();
        assert_eq!(span.line, 3);
        assert_eq!(span.column, 7);
        assert_eq!(span.byte_index, 10);
        assert_eq!(span.length, 1);
    }

    #[test]
    fn drain_errors_is_empty_by_default() {
        let mut lex = Lexer::new("ok");
        lex.next_token().unwrap();
        assert!(lex.drain_errors().is_empty());
    }

    #[test]
    fn lexer_never_panics_on_random_input() {
        // Ensure the lexer handles all sorts of edge-case inputs without panicking.
        let inputs = [
            "", "\0", "\n", "\r\n", "    ", "\t", "\"", "'", "\\", r#""\"""#, r#""\\"#, "#", "`",
            "@", "~", "$", "0x", "0b", "1e10", ".", "..", "...", "==", "!=", "->", "=>", "::",
            ":=", "+=", "-=", "*=", "/=", "%=", "&&", "||", "//", "/*", "*)",
        ];
        for input in &inputs {
            let mut lex = Lexer::new(input);
            loop {
                match lex.next_token() {
                    Ok(tok) if matches!(tok.kind, TokenKind::Eof) => break,
                    Ok(_) => continue,
                    // Break on error to avoid infinite loop (lexer does not advance past errors).
                    Err(_) => break,
                }
            }
        }
    }
}
