use super::token::{Span, Token};
use crate::error::LexError;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    line: usize,
    col: usize,
    indent_stack: Vec<usize>,
    pending_tokens: Vec<(Token, Span)>,
    at_line_start: bool,
    paren_level: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            line: 1,
            col: 1,
            indent_stack: vec![0],
            pending_tokens: Vec::new(),
            at_line_start: true,
            paren_level: 0,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.source.peek().cloned()
    }
    fn advance(&mut self) -> Option<char> {
        let c = self.source.next()?;
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(c)
    }

    pub fn tokenize(&mut self) -> Result<Vec<(Token, Span)>, LexError> {
        let mut tokens = Vec::new();
        loop {
            let (token, span) = self.next_token()?;
            let is_eof = token == Token::Eof;
            tokens.push((token, span));
            if is_eof {
                break;
            }
        }
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<(Token, Span), LexError> {
        if !self.pending_tokens.is_empty() {
            return Ok(self.pending_tokens.remove(0));
        }

        if self.at_line_start {
            self.handle_indentation()?;
            if !self.pending_tokens.is_empty() {
                return Ok(self.pending_tokens.remove(0));
            }
        }

        self.skip_whitespace();
        let start_line = self.line;
        let start_col = self.col;

        let c = match self.advance() {
            Some(c) => c,
            None => {
                while self.indent_stack.len() > 1 {
                    self.indent_stack.pop();
                    self.pending_tokens.push((
                        Token::Dedent,
                        Span {
                            line: start_line,
                            col: start_col,
                            len: 1,
                        },
                    ));
                }
                return if !self.pending_tokens.is_empty() {
                    Ok(self.pending_tokens.remove(0))
                } else {
                    Ok((
                        Token::Eof,
                        Span {
                            line: start_line,
                            col: start_col,
                            len: 1,
                        },
                    ))
                };
            }
        };

        let token = match c {
            '#' => {
                while let Some(pc) = self.peek() {
                    if pc == '\n' {
                        break;
                    }
                    self.advance();
                }
                return self.next_token();
            }
            '\n' => {
                self.at_line_start = true;
                // Only emit Newline if not inside parens
                if self.paren_level == 0 {
                    Token::Newline
                } else {
                    return self.next_token();
                }
            }
            '(' => {
                self.paren_level += 1;
                Token::LParen
            }
            ')' => {
                if self.paren_level > 0 {
                    self.paren_level -= 1;
                }
                Token::RParen
            }
            '{' => {
                self.paren_level += 1;
                Token::LBrace
            }
            '}' => {
                if self.paren_level > 0 {
                    self.paren_level -= 1;
                }
                Token::RBrace
            }
            '[' => {
                self.paren_level += 1;
                Token::LBracket
            }
            ']' => {
                if self.paren_level > 0 {
                    self.paren_level -= 1;
                }
                Token::RBracket
            }
            ',' => Token::Comma,
            ':' => Token::Colon,
            '?' => Token::Question,
            '|' => Token::Pipe,
            '.' => {
                if self.peek() == Some('.') {
                    self.advance();
                    Token::DotDot
                } else {
                    Token::Dot
                }
            }
            '+' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::PlusEq
                } else {
                    Token::Plus
                }
            }
            '-' => match self.peek() {
                Some('=') => {
                    self.advance();
                    Token::MinusEq
                }
                Some('>') => {
                    self.advance();
                    Token::Arrow
                }
                _ => Token::Minus,
            },
            '*' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::StarEq
                } else {
                    Token::Star
                }
            }
            '/' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::SlashEq
                } else {
                    Token::Slash
                }
            }
            '%' => Token::Percent,
            '=' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::EqEq
                } else {
                    Token::Assign
                }
            }
            '!' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::BangEq
                } else {
                    return Err(LexError::new(
                        "Unexpected character '!'",
                        Span {
                            line: start_line,
                            col: start_col,
                            len: 1,
                        },
                    ));
                }
            }
            '<' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::LtEq
                } else {
                    Token::Lt
                }
            }
            '>' => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::GtEq
                } else {
                    Token::Gt
                }
            }
            '"' | '\'' => return self.lex_string(c, start_line, start_col),
            c if c.is_ascii_digit() => self.lex_number(c),
            c if c.is_alphabetic() || c == '_' => self.lex_identifier(c),
            _ => {
                return Err(LexError::new(
                    format!("Unexpected character '{}'", c),
                    Span {
                        line: start_line,
                        col: start_col,
                        len: 1,
                    },
                ))
            }
        };
        let mut len = if self.line == start_line {
            self.col.saturating_sub(start_col)
        } else {
            1
        };
        if len == 0 {
            len = 1;
        }
        Ok((
            token,
            Span {
                line: start_line,
                col: start_col,
                len,
            },
        ))
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c == ' ' || c == '\t' || c == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn handle_indentation(&mut self) -> Result<(), LexError> {
        self.at_line_start = false;
        let mut indent = 0;
        while let Some(c) = self.peek() {
            if c == ' ' {
                indent += 1;
                self.advance();
            } else if c == '\t' {
                indent += 4;
                self.advance();
            } else {
                break;
            }
        }

        // Skip blank lines and comments
        if self.peek() == Some('\n')
            || self.peek() == Some('\r')
            || self.peek() == Some('#')
            || self.peek().is_none()
        {
            return Ok(());
        }

        let last_indent = *self.indent_stack.last().unwrap();
        if indent > last_indent {
            self.indent_stack.push(indent);
            self.pending_tokens.push((
                Token::Indent,
                Span {
                    line: self.line,
                    col: self.col,
                    len: 1,
                },
            ));
        } else if indent < last_indent {
            while indent < *self.indent_stack.last().unwrap() {
                self.indent_stack.pop();
                self.pending_tokens.push((
                    Token::Dedent,
                    Span {
                        line: self.line,
                        col: self.col,
                        len: 1,
                    },
                ));
            }
            if indent != *self.indent_stack.last().unwrap() {
                return Err(LexError::new(
                    format!(
                        "Indentation error: expected {} spaces, got {}",
                        self.indent_stack.last().unwrap(),
                        indent
                    ),
                    Span {
                        line: self.line,
                        col: self.col,
                        len: 1,
                    },
                ));
            }
        }
        Ok(())
    }

    fn lex_string(
        &mut self,
        quote: char,
        start_line: usize,
        start_col: usize,
    ) -> Result<(Token, Span), LexError> {
        let mut s = String::new();
        while let Some(c) = self.advance() {
            if c == quote {
                let mut len = if self.line == start_line {
                    self.col.saturating_sub(start_col)
                } else {
                    1
                };
                if len == 0 {
                    len = 1;
                }
                return Ok((
                    Token::Str(s),
                    Span {
                        line: start_line,
                        col: start_col,
                        len,
                    },
                ));
            }
            if c == '\\' {
                match self.advance() {
                    Some('n') => s.push('\n'),
                    Some('r') => s.push('\r'),
                    Some('t') => s.push('\t'),
                    Some('\\') => s.push('\\'),
                    Some('"') => s.push('"'),
                    Some('\'') => s.push('\''),
                    Some(other) => s.push(other),
                    None => {
                        return Err(LexError::new(
                            "Unterminated string escape",
                            Span {
                                line: start_line,
                                col: start_col,
                                len: 1,
                            },
                        ))
                    }
                }
            } else {
                s.push(c);
            }
        }
        Err(LexError::new(
            "Unterminated string",
            Span {
                line: start_line,
                col: start_col,
                len: 1,
            },
        ))
    }

    fn lex_number(&mut self, first: char) -> Token {
        let mut s = first.to_string();
        let mut is_float = false;
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                s.push(self.advance().unwrap());
            } else if c == '.' && !is_float {
                let mut next_next = self.source.clone();
                next_next.next();
                if next_next.next() == Some('.') {
                    break;
                }
                is_float = true;
                s.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        if is_float {
            Token::Float(s.parse().unwrap())
        } else {
            Token::Int(s.parse().unwrap())
        }
    }

    fn lex_identifier(&mut self, first: char) -> Token {
        let mut s = first.to_string();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                s.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        match s.as_str() {
            "fn" => Token::Fn,
            "cls" => Token::Cls,
            "load" => Token::Load,
            "from" => Token::From,
            "as" => Token::As,
            "export" => Token::Export,
            "return" => Token::Return,
            "if" => Token::If,
            "elif" => Token::Elif,
            "else" => Token::Else,
            "for" => Token::For,
            "while" => Token::While,
            "in" => Token::In,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "spawn" => Token::Spawn,
            "error" => Token::Error,
            "step" => Token::Step,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            "null" => Token::Null,
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,
            _ => Token::Ident(s),
        }
    }
}
