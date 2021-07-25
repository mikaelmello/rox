use std::{fmt::Display, usize};
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
pub enum RuntimeError {
    #[error("Missing operand for operation")]
    MissingOperand,
    #[error("Invalid operand type for operation")]
    InvalidOperand,
    #[error("Invalid constant address")]
    InvalidConstantAddress,
}

#[derive(Error, Debug)]
pub enum RoxErrorKind {
    #[error("{0}")]
    CompilationError(#[from] CompilationError),
    #[error("{0}")]
    RuntimeError(#[from] RuntimeError),
}

#[derive(Error, Debug)]
pub struct RoxError {
    #[source]
    pub src: RoxErrorKind,
    pub line: usize,
}

impl RoxError {
    pub fn new(src: RoxErrorKind, line: usize) -> Self {
        Self { src, line }
    }
}

impl Display for RoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.src)
    }
}

pub type RoxResult<T> = Result<T, RoxError>;
