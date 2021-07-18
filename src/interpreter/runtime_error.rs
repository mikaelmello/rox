use crate::lexer::location::Location;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Operand must be a number")]
    NotNumberOperand(Location),

    #[error("Division by zero")]
    DvisionByZero(Location),

    #[error("'{0}' not supported between instances of '{1}' and '{2}'")]
    OperationNotSupported(String, String, String, Location),
}
