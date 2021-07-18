use std::io;

use super::{lexical_error::LexicalError, token::Token};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScanningError {
    #[error("{0}")]
    LexicalError(#[from] LexicalError),

    #[error("IO Error: {0}")]
    IO(#[from] io::Error),
}

pub type ScanResult = Result<Option<Token>, ScanningError>;
