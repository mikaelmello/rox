use crate::lexer::location::Location;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexicalError {
    #[error("Invalid lexeme \"{0}\"")]
    InvalidLexeme(String, Location),

    #[error("Invalid number literal {0}")]
    InvalidNumberLiteral(String, Location),

    #[error("String literal is not terminated")]
    UnterminatedString(Location),
}
