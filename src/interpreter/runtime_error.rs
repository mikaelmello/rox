use crate::lexer::location::Location;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("{0}: Operand must be a number")]
    NotNumberOperand(Location),

    #[error("'{0}' not supported between instances of '{1}' and '{2}'")]
    OperationNotSupported(String, String, String, Location),
}
