use std::io;

use crate::lexer::{lexical_error::LexicalError, location::Location, scan_result::ScanningError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("{0}")]
    LexicalError(#[from] LexicalError),

    #[error("Missing closing parenthesis at {0}")]
    MissingClosingParenthesis(Location),

    #[error("Unexpected expression \"{1}\" at {0}")]
    UnexpectedExpression(Location, String),

    #[error("Missing expression at {0}")]
    MissingExpression(Location),
}

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
