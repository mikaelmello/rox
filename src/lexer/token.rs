use std::fmt::Display;

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

impl TokenKind {
    pub fn variant_eq(&self, rhs: &TokenKind) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(rhs)
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::LeftParen => write!(f, "{}", "leftparen"),
            TokenKind::RightParen => write!(f, "{}", "rightparen"),
            TokenKind::LeftBrace => write!(f, "{}", "leftbrace"),
            TokenKind::RightBrace => write!(f, "{}", "rightbrace"),
            TokenKind::Comma => write!(f, "{}", "comma"),
            TokenKind::Dot => write!(f, "{}", "dot"),
            TokenKind::Minus => write!(f, "{}", "minus"),
            TokenKind::Plus => write!(f, "{}", "plus"),
            TokenKind::Semicolon => write!(f, "{}", "semicolon"),
            TokenKind::Slash => write!(f, "{}", "slash"),
            TokenKind::Star => write!(f, "{}", "star"),
            TokenKind::Bang => write!(f, "{}", "bang"),
            TokenKind::BangEqual => write!(f, "{}", "bangequal"),
            TokenKind::Equal => write!(f, "{}", "equal"),
            TokenKind::EqualEqual => write!(f, "{}", "equalequal"),
            TokenKind::Greater => write!(f, "{}", "greater"),
            TokenKind::GreaterEqual => write!(f, "{}", "greaterequal"),
            TokenKind::Less => write!(f, "{}", "less"),
            TokenKind::LessEqual => write!(f, "{}", "lessequal"),
            TokenKind::Identifier(_) => write!(f, "{}", "identifier"),
            TokenKind::String(_) => write!(f, "{}", "string"),
            TokenKind::Number(_) => write!(f, "{}", "number"),
            TokenKind::And => write!(f, "{}", "and"),
            TokenKind::Class => write!(f, "{}", "class"),
            TokenKind::Else => write!(f, "{}", "else"),
            TokenKind::False => write!(f, "{}", "false"),
            TokenKind::Fun => write!(f, "{}", "fun"),
            TokenKind::For => write!(f, "{}", "for"),
            TokenKind::If => write!(f, "{}", "if"),
            TokenKind::Nil => write!(f, "{}", "nil"),
            TokenKind::Or => write!(f, "{}", "or"),
            TokenKind::Print => write!(f, "{}", "print"),
            TokenKind::Return => write!(f, "{}", "return"),
            TokenKind::Super => write!(f, "{}", "super"),
            TokenKind::This => write!(f, "{}", "this"),
            TokenKind::True => write!(f, "{}", "true"),
            TokenKind::Var => write!(f, "{}", "var"),
            TokenKind::While => write!(f, "{}", "while"),
        }
    }
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

    pub fn location(&self) -> Location {
        self.loc
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }
}
