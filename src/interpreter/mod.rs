use crate::parser::ast::{BinOp, Expr, Literal, Stmt, UnaryOp};

use self::runtime_error::RuntimeError;

mod runtime_error;

pub trait Interpret {
    fn evaluate(self) -> Result<Literal, RuntimeError>;
}

impl Interpret for Stmt {
    fn evaluate(self) -> Result<Literal, RuntimeError> {
        match self {
            Stmt::Expression(expr) => expr.evaluate(),
            Stmt::Print(expr) => {
                let result = expr.evaluate()?;
                println!("{}", result);
                Ok(Literal::Nil(result.location()))
            }
        }
    }
}

impl Interpret for Expr {
    fn evaluate(self) -> Result<Literal, RuntimeError> {
        match self {
            Expr::Binary(left, op, right) => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;

                macro_rules! not_supported {
                    ($op:expr,$lhs:expr,$rhs:expr) => {{
                        Err(RuntimeError::OperationNotSupported(
                            $op.symbol(),
                            $lhs.symbol(),
                            $rhs.symbol(),
                            $lhs.location(),
                        ))
                    }};
                }

                match op {
                    BinOp::Plus => match (left, right) {
                        (Literal::Number(l, loc), Literal::Number(r, _)) => {
                            Ok(Literal::Number(l + r, loc))
                        }
                        (Literal::String(l, loc), Literal::String(r, _)) => {
                            Ok(Literal::String(l + &r, loc))
                        }
                        (Literal::String(l, loc), rhs) => {
                            Ok(Literal::String(l + &rhs.to_string(), loc))
                        }
                        (lhs, Literal::String(r, loc)) => {
                            Ok(Literal::String(lhs.to_string() + &r, loc))
                        }
                        (lhs, rhs) => not_supported!(op, lhs, rhs),
                    },
                    BinOp::Minus => match (left, right) {
                        (Literal::Number(l, loc), Literal::Number(r, _)) => {
                            Ok(Literal::Number(l - r, loc))
                        }
                        (lhs, rhs) => not_supported!(op, lhs, rhs),
                    },
                    BinOp::Star => match (left, right) {
                        (Literal::Number(l, loc), Literal::Number(r, _)) => {
                            Ok(Literal::Number(l * r, loc))
                        }
                        (lhs, rhs) => not_supported!(op, lhs, rhs),
                    },
                    BinOp::Slash => match (left, right) {
                        (Literal::Number(_, _), Literal::Number(r, loc)) if r == 0f64 => {
                            Err(RuntimeError::DvisionByZero(loc))
                        }
                        (Literal::Number(l, loc), Literal::Number(r, _)) => {
                            Ok(Literal::Number(l / r, loc))
                        }
                        (lhs, rhs) => not_supported!(op, lhs, rhs),
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
                        (lhs, rhs) => not_supported!(op, lhs, rhs),
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
                        (lhs, rhs) => not_supported!(op, lhs, rhs),
                    },
                    BinOp::Greater => match (left, right) {
                        (Literal::Number(l, loc), Literal::Number(r, _)) => {
                            Ok(Literal::Bool(l > r, loc))
                        }
                        (lhs, rhs) => not_supported!(op, lhs, rhs),
                    },
                    BinOp::GreaterEqual => match (left, right) {
                        (Literal::Number(l, loc), Literal::Number(r, _)) => {
                            Ok(Literal::Bool(l >= r, loc))
                        }
                        (lhs, rhs) => not_supported!(op, lhs, rhs),
                    },
                    BinOp::Less => match (left, right) {
                        (Literal::Number(l, loc), Literal::Number(r, _)) => {
                            Ok(Literal::Bool(l < r, loc))
                        }
                        (lhs, rhs) => not_supported!(op, lhs, rhs),
                    },
                    BinOp::LessEqual => match (left, right) {
                        (Literal::Number(l, loc), Literal::Number(r, _)) => {
                            Ok(Literal::Bool(l <= r, loc))
                        }
                        (lhs, rhs) => not_supported!(op, lhs, rhs),
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
