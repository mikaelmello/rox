use core::panic;
use std::fmt::Debug;

use crate::lexer::token::{Token, TokenKind};

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
    Bool(bool),
    Number(f64),
    String(String),
    Nil,
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
