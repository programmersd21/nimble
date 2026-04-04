#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Null,

    // Identifiers & Keywords
    Ident(String),
    Fn,
    Cls,
    Load,
    From,
    As,
    Export,
    Return,
    If,
    Elif,
    Else,
    For,
    While,
    In,
    Break,
    Continue,
    Spawn,
    Error,
    Step,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Assign,
    EqEq,
    BangEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
    Not,
    Arrow,
    Question,
    DotDot,
    Dot,
    Pipe,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,

    // Structure
    Indent,
    Dedent,
    Newline,
    Colon,
    Comma,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    // Interp
    InterpStart,
    InterpEnd,

    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub line: usize,
    pub col: usize,
    pub len: usize,
}

impl Default for Span {
    fn default() -> Self {
        Self {
            line: 1,
            col: 1,
            len: 1,
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Token::Eof
    }
}
