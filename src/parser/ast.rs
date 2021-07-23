use core::panic;
use std::fmt::{Debug, Display};

use crate::lexer::{
    location::Location,
    token::{Token, TokenKind},
};

pub enum Stmt {
    Expression(Expr),
    Print(Expr),
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

impl BinOp {
    pub fn symbol(&self) -> String {
        match self {
            BinOp::Minus => String::from("-"),
            BinOp::Plus => String::from("+"),
            BinOp::Slash => String::from("/"),
            BinOp::Star => String::from("*"),
            BinOp::BangEqual => String::from("!="),
            BinOp::EqualEqual => String::from("=="),
            BinOp::Greater => String::from(">"),
            BinOp::GreaterEqual => String::from(">="),
            BinOp::Less => String::from("<"),
            BinOp::LessEqual => String::from("<="),
        }
    }
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
    pub fn symbol(&self) -> String {
        match self {
            Literal::Bool(_, _) => String::from("bool"),
            Literal::Number(_, _) => String::from("number"),
            Literal::String(_, _) => String::from("string"),
            Literal::Nil(_) => String::from("nil"),
        }
    }

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
            Literal::Nil(_) => Ok(()),
        }
    }
}
