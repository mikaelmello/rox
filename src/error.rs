use thiserror::Error;

#[derive(Error, Debug)]
pub enum RoxError {
    #[error("Compilation error")]
    CompileError,
    #[error("Runtime error")]
    RuntimeError,
}

pub type RoxResult<T> = Result<T, RoxError>;
