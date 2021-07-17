use std::io;

use thiserror::Error;

use crate::lexer::location::Location;

#[derive(Error, Debug)]
pub enum LexicalError {
    #[error("Invalid lexeme {1}: {0}")]
    InvalidLexeme(String, Location),
    #[error("Invalid number literal {1}: {0}")]
    InvalidNumberLiteral(String, Location),
    #[error("String at {0} is not terminated")]
    UnterminatedString(Location),
    #[error("Error with IO operations")]
    IO(#[from] io::Error),
}
