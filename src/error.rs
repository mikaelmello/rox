pub enum RoxError {
    CompileError,
    RuntimeError,
}

pub type RoxResult<T> = Result<T, RoxError>;
