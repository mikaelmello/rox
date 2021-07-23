use std::io;

use crate::lexer::{
    lexical_error::LexicalError, location::Location, scan_result::ScanningError, token::TokenKind,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("{0}")]
    LexicalError(#[from] LexicalError),

    #[error("Missing closing parenthesis")]
    MissingClosingParenthesis(Location),

    #[error("Unexpected expression \"{1}\"")]
    UnexpectedExpression(Location, String),

    #[error("{1}")]
    ExpectedToken(Location, String),

    #[error("Missing expression")]
    MissingExpression(Location),
}

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("{0}")]
    SyntaxError(#[from] SyntaxError),

    #[error("IO Error: {0}")]
    IO(#[from] io::Error),
}

impl From<ScanningError> for ParseError {
    fn from(err: ScanningError) -> Self {
        match err {
            ScanningError::IO(err) => ParseError::IO(err),
            ScanningError::LexicalError(err) => ParseError::SyntaxError(err.into()),
        }
    }
}
