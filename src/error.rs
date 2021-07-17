use std::io;

use thiserror::Error;

use crate::lexer::lexical_error::LexicalError;

#[derive(Error, Debug)]
pub enum RoxError {
    #[error("Error with IO operations")]
    IO(#[from] io::Error),

    #[error("{0}")]
    LexicalError(#[from] LexicalError),
}
