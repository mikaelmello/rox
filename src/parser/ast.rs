use std::fmt::Debug;

use crate::lexer::token::Token;

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(Token, Box<Expr>),
}

#[derive(Debug)]
pub enum Literal {
    Bool(bool),
    Number(f64),
    String(String),
    Nil,
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(left, op, right) => write!(f, "({:?} {:?} {:?})", op, left, right),
            Expr::Grouping(expression) => write!(f, "(group {:?})", expression),
            Expr::Literal(value) => write!(f, "{:?}", value),
            Expr::Unary(op, expression) => write!(f, "({:?} {:?})", op, expression),
        }
    }
}
