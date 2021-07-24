use self::runtime_error::RuntimeError;
use crate::parser::ast::{BinOp, Expr, Literal, LiteralKind, Stmt, UnaryOp};

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
                Ok(Literal::new(LiteralKind::Nil, result.location()))
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
                    BinOp::Plus => match (left.literal(), right.literal()) {
                        (LiteralKind::Number(l), LiteralKind::Number(r)) => {
                            Ok(Literal::new(LiteralKind::Number(l + r), left.location()))
                        }
                        (LiteralKind::String(l), LiteralKind::String(r)) => Ok(Literal::new(
                            LiteralKind::String(l.clone() + r),
                            left.location(),
                        )),
                        (LiteralKind::String(l), _) => Ok(Literal::new(
                            LiteralKind::String(l.clone() + &right.to_string()),
                            left.location(),
                        )),
                        (_, LiteralKind::String(r)) => Ok(Literal::new(
                            LiteralKind::String(left.to_string() + r),
                            left.location(),
                        )),
                        (_, _) => not_supported!(op, left, right),
                    },
                    BinOp::Minus => match (left.literal(), right.literal()) {
                        (LiteralKind::Number(l), LiteralKind::Number(r)) => {
                            Ok(Literal::new(LiteralKind::Number(l - r), left.location()))
                        }
                        (_, _) => not_supported!(op, left, right),
                    },
                    BinOp::Star => match (left.literal(), right.literal()) {
                        (LiteralKind::Number(l), LiteralKind::Number(r)) => {
                            Ok(Literal::new(LiteralKind::Number(l * r), left.location()))
                        }
                        (_, _) => not_supported!(op, left, right),
                    },
                    BinOp::Slash => match (left.literal(), right.literal()) {
                        (LiteralKind::Number(_), LiteralKind::Number(r)) if *r == 0f64 => {
                            Err(RuntimeError::DvisionByZero(right.location()))
                        }
                        (LiteralKind::Number(l), LiteralKind::Number(r)) => {
                            Ok(Literal::new(LiteralKind::Number(l / r), left.location()))
                        }
                        (_, _) => not_supported!(op, left, right),
                    },
                    BinOp::BangEqual => match (left.literal(), right.literal()) {
                        (LiteralKind::Number(l), LiteralKind::Number(r)) => {
                            Ok(Literal::new(LiteralKind::Bool(l != r), left.location()))
                        }
                        (LiteralKind::Bool(l), LiteralKind::Bool(r)) => {
                            Ok(Literal::new(LiteralKind::Bool(l != r), left.location()))
                        }
                        (LiteralKind::String(l), LiteralKind::String(r)) => {
                            Ok(Literal::new(LiteralKind::Bool(l != r), left.location()))
                        }
                        (LiteralKind::Nil, LiteralKind::Nil) => {
                            Ok(Literal::new(LiteralKind::Bool(false), left.location()))
                        }
                        (_, _) => not_supported!(op, left, right),
                    },
                    BinOp::EqualEqual => match (left.literal(), right.literal()) {
                        (LiteralKind::Number(l), LiteralKind::Number(r)) => {
                            Ok(Literal::new(LiteralKind::Bool(l == r), left.location()))
                        }
                        (LiteralKind::Bool(l), LiteralKind::Bool(r)) => {
                            Ok(Literal::new(LiteralKind::Bool(l == r), left.location()))
                        }
                        (LiteralKind::String(l), LiteralKind::String(r)) => {
                            Ok(Literal::new(LiteralKind::Bool(l == r), left.location()))
                        }
                        (LiteralKind::Nil, LiteralKind::Nil) => {
                            Ok(Literal::new(LiteralKind::Bool(true), left.location()))
                        }
                        (_, _) => not_supported!(op, left, right),
                    },
                    BinOp::Greater => match (left.literal(), right.literal()) {
                        (LiteralKind::Number(l), LiteralKind::Number(r)) => {
                            Ok(Literal::new(LiteralKind::Bool(l > r), left.location()))
                        }
                        (_, _) => not_supported!(op, left, right),
                    },
                    BinOp::GreaterEqual => match (left.literal(), right.literal()) {
                        (LiteralKind::Number(l), LiteralKind::Number(r)) => {
                            Ok(Literal::new(LiteralKind::Bool(l >= r), left.location()))
                        }
                        (_, _) => not_supported!(op, left, right),
                    },
                    BinOp::Less => match (left.literal(), right.literal()) {
                        (LiteralKind::Number(l), LiteralKind::Number(r)) => {
                            Ok(Literal::new(LiteralKind::Bool(l < r), left.location()))
                        }
                        (_, _) => not_supported!(op, left, right),
                    },
                    BinOp::LessEqual => match (left.literal(), right.literal()) {
                        (LiteralKind::Number(l), LiteralKind::Number(r)) => {
                            Ok(Literal::new(LiteralKind::Bool(l <= r), left.location()))
                        }
                        (_, _) => not_supported!(op, left, right),
                    },
                }
            }
            Expr::Grouping(expr) => expr.evaluate(),
            Expr::Literal(val) => Ok(val),
            Expr::Unary(op, expr) => {
                let val = expr.evaluate()?;

                match op {
                    UnaryOp::Bang => Ok(Literal::new(
                        LiteralKind::Bool(!val.is_truthy()),
                        val.location(),
                    )),
                    UnaryOp::Minus => match val.literal() {
                        LiteralKind::Number(n) => {
                            Ok(Literal::new(LiteralKind::Number(-n), val.location()))
                        }
                        _ => Err(RuntimeError::NotNumberOperand(val.location())),
                    },
                }
            }
        }
    }
}
