use std::fmt::Display;

use crate::location::Location;
use thiserror::Error;

#[derive(Error, Debug)]
pub struct ParseError {
    #[source]
    src: ParseErrorKind,
    loc: Location,
}

impl ParseError {
    pub fn new(src: ParseErrorKind, loc: Location) -> Self {
        Self { src, loc }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error {}\n    {}\n", self.loc, self.src)
    }
}

#[derive(Error, Debug)]
pub enum ParseErrorKind {
    #[error("Invalid lexeme \"{0}\"")]
    InvalidLexeme(String),

    #[error("Invalid number literal {0}")]
    InvalidNumberLiteral(String),

    #[error("String literal is not properly formatted")]
    InvalidStringLiteral,

    #[error("String literal is not terminated")]
    UnterminatedString,

    #[error("Missing closing parenthesis")]
    MissingClosingParenthesis,

    #[error("Unexpected expression \"{0}\"")]
    UnexpectedExpression(String),

    #[error("{0}")]
    ExpectedToken(String),

    #[error("Missing expression")]
    MissingExpression,
}

pub type ParseResult<T> = Result<T, ParseError>;
