use crate::{
    location::Location,
    scanner::{Token, TokenKind},
};
use core::panic;
use std::fmt::{Debug, Display};

pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(Identifier, Option<Expr>),
}

pub enum Expr {
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(UnaryOp, Box<Expr>),
    Variable(Identifier),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(left, op, right) => write!(f, "({:?} {:?} {:?})", op, left, right),
            Expr::Grouping(expression) => write!(f, "(group {:?})", expression),
            Expr::Literal(value) => write!(f, "{:?}", value),
            Expr::Unary(op, expression) => write!(f, "({:?} {:?})", op, expression),
            Expr::Variable(name) => write!(f, "{:?}", name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Identifier(String, Location);

impl Identifier {
    pub fn location(&self) -> Location {
        self.1
    }
}

impl<'sourcecode> From<Token<'sourcecode>> for Identifier {
    fn from(t: Token) -> Self {
        match t.kind() {
            TokenKind::Identifier => Identifier(t.lexeme().to_string(), t.location()),
            _ => panic!("Should not be called"),
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
    pub fn symbol(&self) -> &'static str {
        match self {
            BinOp::Minus => "-",
            BinOp::Plus => "+",
            BinOp::Slash => "/",
            BinOp::Star => "*",
            BinOp::BangEqual => "!=",
            BinOp::EqualEqual => "==",
            BinOp::Greater => ">",
            BinOp::GreaterEqual => ">=",
            BinOp::Less => "<",
            BinOp::LessEqual => "<=",
        }
    }
}

impl<'sourcecode> From<Token<'sourcecode>> for BinOp {
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

impl<'sourcecode> From<Token<'sourcecode>> for UnaryOp {
    fn from(t: Token) -> Self {
        match t.kind() {
            TokenKind::Minus => UnaryOp::Minus,
            TokenKind::Bang => UnaryOp::Bang,
            _ => panic!("Should not be called"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralKind {
    Bool(bool),
    Number(f64),
    String(String),
    Nil,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Literal {
    loc: Location,
    literal: LiteralKind,
}

impl Literal {
    pub fn new(literal: LiteralKind, loc: Location) -> Self {
        Self { loc, literal }
    }

    pub fn symbol(&self) -> &'static str {
        match self.literal {
            LiteralKind::Bool(_) => "bool",
            LiteralKind::Number(_) => "number",
            LiteralKind::String(_) => "string",
            LiteralKind::Nil => "nil",
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self.literal {
            LiteralKind::Bool(b) => b,
            LiteralKind::Number(_) => true,
            LiteralKind::String(_) => true,
            LiteralKind::Nil => false,
        }
    }

    pub fn location(&self) -> Location {
        self.loc
    }

    pub fn literal(&self) -> &LiteralKind {
        &self.literal
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.literal {
            LiteralKind::Bool(val) => write!(f, "{}", *val),
            LiteralKind::Number(val) => write!(f, "{}", *val),
            LiteralKind::String(val) => write!(f, "{}", *val),
            LiteralKind::Nil => Ok(()),
        }
    }
}
