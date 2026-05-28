use std::collections::VecDeque;

/// `line`/`column` are 1‑based; `byte_index` is a 0‑based byte offset; `length` is the byte length (0 for virtual tokens).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq)]
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
    True,
    False,

    Identifier(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),

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

    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub const fn new(kind: TokenKind, span: Span) -> Self {
        Token { kind, span }
    }
}

/// Tabs are rejected in indentation.
fn count_leading_whitespace(s: &str) -> usize {
    s.bytes().take_while(|&b| b == b' ').count()
}

fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

fn is_ident_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
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

    pub fn next_token(&mut self) -> Result<Token, String> {
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

    pub fn tokenize_all(&mut self) -> Result<Vec<Token>, String> {
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

    fn flush_dedents_at_eof(&mut self) -> Result<(), String> {
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

    fn handle_indentation(&mut self) -> Result<(), String> {
        let raw = self.current_line();
        let indent = count_leading_whitespace(raw);

        let top = *self.indent_stack.last().unwrap();

        if indent > top {
            self.indent_stack.push(indent);
            let span = Span::new(self.line_num, 1, self.current_line_start());
            self.pending.push_back(Token::new(TokenKind::Indent, span));
        } else if indent < top {
            // Pop until the level matches.
            while *self.indent_stack.last().unwrap() > indent {
                self.indent_stack.pop().unwrap();
                let span = Span::new(self.line_num, 1, self.current_line_start());
                self.pending.push_back(Token::new(TokenKind::Dedent, span));
            }
            if self.indent_stack.is_empty() {
                self.indent_stack.push(0);
            }
            // Validate that we landed on an exact match.
            if *self.indent_stack.last().unwrap() != indent {
                if indent != 0 || self.indent_stack.len() != 1 {
                    return Err(format!(
                        "Indentation error at line {}: indent level {} \
                         does not match any enclosing block",
                        self.line_num, indent,
                    ));
                }
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
                b' ' => { self.pos += 1; self.col += 1; }
                b'\t' => return,
                _ => break,
            }
        }
    }

    /// Tokenise the next lexeme at [`self.pos`].
    fn tokenize_next(&mut self) -> Result<Token, String> {
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

            b'\t' => {
                return Err(format!(
                    "Illegal tab character at line {}, column {}",
                    self.line_num, self.col,
                ));
            }

            b'0'..=b'9' => return self.tokenize_number(),
            b'"' => return self.tokenize_string(),
            _ if is_ident_start(ch) => return self.tokenize_identifier_or_keyword(),

            _ => {
                return Err(format!(
                    "Unexpected character '{}' (byte 0x{:02x}) at line {}, column {}",
                    ch as char, ch, self.line_num, self.col,
                ));
            }
        };

        let span = self.span_at_current_pos(width);

        match kind {
            TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace => {
                self.delimiter_count += 1
            }
            TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace => {
                if self.delimiter_count == 0 {
                    return Err(format!(
                        "Unmatched closing delimiter at line {}, column {}",
                        self.line_num, self.col,
                    ));
                }
                self.delimiter_count -= 1;
            }
            _ => {}
        }

        self.advance_pos(width);
        Ok(Token::new(kind, span))
    }

    fn tokenize_number(&mut self) -> Result<Token, String> {
        let start_pos = self.pos;
        let bytes = self.current_line().as_bytes();

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
            let value: f64 = raw
                .parse()
                .map_err(|_| format!("Invalid float literal '{}'", raw))?;
            let len = self.pos - start_pos;
            let span = Span::new_with_len(
                self.line_num,
                start_pos + 1,
                self.current_line_start() + start_pos,
                len,
            );
            self.col += len;
            Ok(Token::new(TokenKind::FloatLiteral(value), span))
        } else {
            let raw = &self.current_line()[start_pos..self.pos];
            let value: i64 = raw
                .parse()
                .map_err(|_| format!("Integer literal '{}' out of range", raw))?;
            let len = self.pos - start_pos;
            let span = Span::new_with_len(
                self.line_num,
                start_pos + 1,
                self.current_line_start() + start_pos,
                len,
            );
            self.col += len;
            Ok(Token::new(TokenKind::IntLiteral(value), span))
        }
    }

    fn tokenize_string(&mut self) -> Result<Token, String> {
        let start_pos = self.pos;
        let byte_offset = self.current_line_start() + start_pos;
        self.pos += 1;
        self.col += 1;

        let mut value = String::new();
        let bytes = self.current_line().as_bytes();

        loop {
            if self.pos >= bytes.len() {
                return Err(format!(
                    "Unterminated string literal starting at line {}, column {}",
                    self.line_num,
                    start_pos + 1,
                ));
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
                        return Err(format!(
                            "Unterminated string escape at line {}, column {}",
                            self.line_num, self.col,
                        ));
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
                            return Err(format!(
                                "Invalid escape sequence '\\{}' at line {}, column {}",
                                other as char, self.line_num, self.col,
                            ));
                        }
                    };
                    value.push(escaped);
                    self.pos += 1;
                    self.col += 1;
                }
                b'\n' | b'\r' => {
                    return Err(format!(
                        "Newline inside string literal at line {}, column {}",
                        self.line_num, self.col,
                    ));
                }
                _ => {
                    value.push(bytes[self.pos] as char);
                    self.pos += 1;
                    self.col += 1;
                }
            }
        }
    }

    fn tokenize_identifier_or_keyword(&mut self) -> Result<Token, String> {
        let start_pos = self.pos;
        let bytes = self.current_line().as_bytes();

        while self.pos < bytes.len() && is_ident_continue(bytes[self.pos]) {
            self.pos += 1;
        }

        let raw = &self.current_line()[start_pos..self.pos];
        let len = self.pos - start_pos;
        let span = Span::new_with_len(
            self.line_num,
            start_pos + 1,
            self.current_line_start() + start_pos,
            len,
        );
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
    type Item = Result<Token, String>;

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

        // Advance until newline or end.
        while pos < src_len && bytes[pos] != b'\n' && bytes[pos] != b'\r' {
            pos += 1;
        }

        lines.push(&source[line_start..pos]);

        // Consume the newline sequence.
        if pos < src_len && bytes[pos] == b'\r' {
            pos += 1;
        }
        if pos < src_len && bytes[pos] == b'\n' {
            pos += 1;
        }
    }

    // If the source ends with a newline, append an empty sentinel line
    // so that the final NEWLINE token is emitted for the line before it.
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
            "fn let var if elif else struct interface pub return in extern for while true false",
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
                TokenKind::Return,
                TokenKind::In,
                TokenKind::Extern,
                TokenKind::For,
                TokenKind::While,
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
    fn unterminated_string_error() {
        let mut lex = Lexer::new(r#""hello"#);
        assert!(lex.next_token().is_err());
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
        assert!(lex.next_token().is_err()); // #
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
        assert!(lex.next_token().is_err()); // error on "  c"
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
        assert!(lex.next_token().is_err());
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
        // No dedent tokens at EOF – they are emitted by EOF handling.
        let src = "a\n    b";
        let toks = kinds_no_eof(src);
        assert_eq!(
            toks,
            vec![
                TokenKind::Identifier("a".into()),
                TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Identifier("b".into()),
                // No Newline after b (file ends)
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
        let src = "\t42";
        let mut lex = Lexer::new(src);
        assert!(lex.next_token().is_err());
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
}
