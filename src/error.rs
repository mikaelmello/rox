use std::io;

use thiserror::Error;

use crate::lexer::location::Location;

#[derive(Error, Debug)]
pub enum RoxError {
    #[error("Error with IO operations")]
    IO(#[from] io::Error),

    #[error("Error {1} {0}")]
    LexicalError(String, Location),
}
