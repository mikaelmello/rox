use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RoxError {
    #[error("Error with IO operations")]
    IO(#[from] io::Error),

    #[error("Syntax error at line {0}: {1}")]
    SyntaxError(usize, String),

    #[error("Unknown error")]
    Unknown,
}
