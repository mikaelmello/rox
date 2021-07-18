use super::location::Location;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    loc: Location,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: &str, loc: Location) -> Self {
        Self {
            kind,
            lexeme: String::from(lexeme),
            loc,
        }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }
}
