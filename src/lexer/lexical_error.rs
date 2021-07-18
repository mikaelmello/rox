use crate::lexer::location::Location;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexicalError {
    #[error("Invalid lexeme {1}: {0}")]
    InvalidLexeme(String, Location),

    #[error("Invalid number literal {1}: {0}")]
    InvalidNumberLiteral(String, Location),

    #[error("String at {0} is not terminated")]
    UnterminatedString(Location),
}
