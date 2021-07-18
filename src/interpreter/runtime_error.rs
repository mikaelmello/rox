use crate::lexer::location::Location;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Operands must be of same type")]
    DifferentTypeOperands(Location),

    #[error("Operands must be numbers")]
    NotNumberOperands(Location),

    #[error("Operand must be a number")]
    NotNumberOperand(Location),

    #[error("Operands must be numbers or strings")]
    NotNumberOrStringOperands(Location),
}
