use super::{lexical_error::LexicalError, token::Token};

pub type ScanResult = Result<Token, LexicalError>;
