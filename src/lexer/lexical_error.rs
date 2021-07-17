use std::io;

use thiserror::Error;

use crate::lexer::location::Location;

#[derive(Error, Debug)]
pub enum LexicalError {
    #[error("Invalid string of characters {1}: {0}")]
    UnrecognizedLexeme(String, Location),
    #[error("Invalid number literal {1}: {0}")]
    InvalidNumberLiteral(String, Location),
    #[error("String at {0} is not terminated")]
    UnterminatedString(Location),
}
