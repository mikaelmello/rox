use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RoxError {
    #[error("Error with IO operations")]
    IO(#[from] io::Error),

    #[error("Unknown error")]
    Unknown,
}
