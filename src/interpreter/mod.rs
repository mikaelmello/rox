use crate::parser::ast::{BinOp, Expr, Literal, UnaryOp};

use self::runtime_error::RuntimeError;

mod runtime_error;

impl Expr {
    pub fn evaluate(self) -> Result<Literal, RuntimeError> {
        match self {
            Expr::Binary(left, op, right) => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;

                match op {
                    BinOp::Plus => match (left, right) {
                        (Literal::Number(l, loc), Literal::Number(r, _)) => {
                            Ok(Literal::Number(l + r, loc))
                        }
                        (Literal::String(l, loc), Literal::String(r, _)) => {
                            Ok(Literal::String(l + &r, loc))
                        }
                        (left, _) => {
                            return Err(RuntimeError::NotNumberOrStringOperands(left.location()))
                        }
                    },
                    BinOp::Minus => match (left, right) {
                        (Literal::Number(l, loc), Literal::Number(r, _)) => {
                            Ok(Literal::Number(l - r, loc))
                        }
                        (left, _) => return Err(RuntimeError::NotNumberOperands(left.location())),
                    },
                    BinOp::Star => match (left, right) {
                        (Literal::Number(l, loc), Literal::Number(r, _)) => {
                            Ok(Literal::Number(l * r, loc))
                        }
                        (left, _) => return Err(RuntimeError::NotNumberOperands(left.location())),
                    },
                    BinOp::Slash => match (left, right) {
                        (Literal::Number(l, loc), Literal::Number(r, _)) => {
                            Ok(Literal::Number(l / r, loc))
                        }
                        (left, _) => return Err(RuntimeError::NotNumberOperands(left.location())),
                    },
                    BinOp::BangEqual => match (left, right) {
                        (Literal::Number(l, loc), Literal::Number(r, _)) => {
                            Ok(Literal::Bool(l != r, loc))
                        }
                        (Literal::Bool(l, loc), Literal::Bool(r, _)) => {
                            Ok(Literal::Bool(l != r, loc))
                        }
                        (Literal::String(l, loc), Literal::String(r, _)) => {
                            Ok(Literal::Bool(l != r, loc))
                        }
                        (Literal::Nil(loc), Literal::Nil(_)) => Ok(Literal::Bool(false, loc)),

                        (left, _) => {
                            return Err(RuntimeError::DifferentTypeOperands(left.location()))
                        }
                    },
                    BinOp::EqualEqual => match (left, right) {
                        (Literal::Number(l, loc), Literal::Number(r, _)) => {
                            Ok(Literal::Bool(l == r, loc))
                        }
                        (Literal::Bool(l, loc), Literal::Bool(r, _)) => {
                            Ok(Literal::Bool(l == r, loc))
                        }
                        (Literal::String(l, loc), Literal::String(r, _)) => {
                            Ok(Literal::Bool(l == r, loc))
                        }
                        (Literal::Nil(loc), Literal::Nil(_)) => Ok(Literal::Bool(true, loc)),

                        (left, _) => {
                            return Err(RuntimeError::DifferentTypeOperands(left.location()))
                        }
                    },
                    BinOp::Greater => match (left, right) {
                        (Literal::Number(l, loc), Literal::Number(r, _)) => {
                            Ok(Literal::Bool(l > r, loc))
                        }
                        (left, _) => return Err(RuntimeError::NotNumberOperands(left.location())),
                    },
                    BinOp::GreaterEqual => match (left, right) {
                        (Literal::Number(l, loc), Literal::Number(r, _)) => {
                            Ok(Literal::Bool(l >= r, loc))
                        }
                        (left, _) => return Err(RuntimeError::NotNumberOperands(left.location())),
                    },
                    BinOp::Less => match (left, right) {
                        (Literal::Number(l, loc), Literal::Number(r, _)) => {
                            Ok(Literal::Bool(l < r, loc))
                        }
                        (left, _) => return Err(RuntimeError::NotNumberOperands(left.location())),
                    },
                    BinOp::LessEqual => match (left, right) {
                        (Literal::Number(l, loc), Literal::Number(r, _)) => {
                            Ok(Literal::Bool(l <= r, loc))
                        }
                        (left, _) => return Err(RuntimeError::NotNumberOperands(left.location())),
                    },
                }
            }
            Expr::Grouping(expr) => expr.evaluate(),
            Expr::Literal(val) => Ok(val),
            Expr::Unary(op, expr) => {
                let val = expr.evaluate()?;

                match op {
                    UnaryOp::Bang => Ok(Literal::Bool(!val.is_truthy(), val.location())),
                    UnaryOp::Minus => match val {
                        Literal::Number(n, l) => Ok(Literal::Number(-n, l)),
                        l => Err(RuntimeError::NotNumberOperand(l.location())),
                    },
                }
            }
        }
    }
}
