use core::panic;
use std::fmt::{Debug, Display};

use crate::lexer::{
    location::Location,
    token::{Token, TokenKind},
};

#[derive(Copy, Clone, Debug)]
pub enum BinOp {
    Plus,
    Minus,
    Star,
    Slash,
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl From<Token> for BinOp {
    fn from(t: Token) -> Self {
        match t.kind() {
            TokenKind::Minus => BinOp::Minus,
            TokenKind::Plus => BinOp::Plus,
            TokenKind::Slash => BinOp::Slash,
            TokenKind::Star => BinOp::Star,
            TokenKind::BangEqual => BinOp::BangEqual,
            TokenKind::EqualEqual => BinOp::EqualEqual,
            TokenKind::Greater => BinOp::Greater,
            TokenKind::GreaterEqual => BinOp::GreaterEqual,
            TokenKind::Less => BinOp::Less,
            TokenKind::LessEqual => BinOp::LessEqual,
            _ => panic!("Should not be called"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum UnaryOp {
    Minus,
    Bang,
}

impl From<Token> for UnaryOp {
    fn from(t: Token) -> Self {
        match t.kind() {
            TokenKind::Minus => UnaryOp::Minus,
            TokenKind::Bang => UnaryOp::Bang,
            _ => panic!("Should not be called"),
        }
    }
}

#[derive(Debug)]
pub enum Literal {
    Bool(bool, Location),
    Number(f64, Location),
    String(String, Location),
    Nil(Location),
}

impl Literal {
    pub fn is_truthy(&self) -> bool {
        match self {
            Literal::Bool(b, _) => *b,
            Literal::Number(_, _) => true,
            Literal::String(_, _) => true,
            Literal::Nil(_) => false,
        }
    }

    pub fn location(&self) -> Location {
        match self {
            Literal::Bool(_, l) => *l,
            Literal::Number(_, l) => *l,
            Literal::String(_, l) => *l,
            Literal::Nil(l) => *l,
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Bool(val, _) => write!(f, "{}", val),
            Literal::Number(val, _) => write!(f, "{}", val),
            Literal::String(val, _) => write!(f, "{}", val),
            Literal::Nil(_) => write!(f, "nil"),
        }
    }
}

pub enum Expr {
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(UnaryOp, Box<Expr>),
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
