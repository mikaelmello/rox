use crate::location::Location;
use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilationError {
    #[error("Invalid lexeme \"{0}\"")]
    InvalidLexeme(String),

    #[error("Invalid number literal {0}")]
    InvalidNumberLiteral(String),

    #[error("String literal is not terminated")]
    UnterminatedString,

    #[error("Missing closing parenthesis")]
    MissingClosingParenthesis,

    #[error("Too many constants in one code section, limit is {0}")]
    TooManyConstants(u64),

    #[error("Missing expression")]
    MissingExpression,
}

#[derive(Error, Debug)]
pub enum RoxErrorKind {
    #[error("{0}")]
    CompilationError(#[from] CompilationError),
    #[error("Runtime error")]
    RuntimeError,
}

#[derive(Error, Debug)]
pub struct RoxError {
    #[source]
    pub src: RoxErrorKind,
    pub loc: Location,
}

impl RoxError {
    pub fn new(src: RoxErrorKind, loc: Location) -> Self {
        Self { src, loc }
    }
}

impl Display for RoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error {}\n    {}\n", self.loc, self.src)
    }
}

pub type RoxResult<T> = Result<T, RoxError>;
