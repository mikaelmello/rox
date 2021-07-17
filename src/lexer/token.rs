use super::location::Location;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
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
    r#type: TokenType,
    lexeme: String,
    loc: Location,
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: &str, loc: Location) -> Self {
        Self {
            r#type,
            lexeme: String::from(lexeme),
            loc,
        }
    }

    pub fn r#type(&self) -> &TokenType {
        &self.r#type
    }
}
