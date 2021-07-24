use thiserror::Error;

use crate::location::Location;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Operand must be a number")]
    NotNumberOperand(Location),

    #[error("Division by zero")]
    DvisionByZero(Location),

    #[error("'{0}' not supported between instances of '{1}' and '{2}'")]
    OperationNotSupported(&'static str, &'static str, &'static str, Location),
}
