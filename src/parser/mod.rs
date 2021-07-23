use crate::{
    lexer::{
        location::Location,
        scan_result::ScanningError,
        scanner::TokenIter,
        token::{Token, TokenKind},
    },
    parser::ast::Literal,
};
use std::{
    collections::VecDeque,
    io::{Read, Seek},
};

use self::{
    ast::{Expr, Stmt},
    parse_error::{ParseError, ParseResult, SyntaxError},
};

pub mod ast;
mod parse_error;

pub struct Parser<T: Read + Seek> {
    scanner: TokenIter<T>,
    buffer: VecDeque<Token>,
}

impl<T: Read + Seek> Parser<T> {
    pub fn new(scanner: TokenIter<T>) -> Self {
        Self {
            scanner,
            buffer: VecDeque::new(),
        }
    }

    pub fn parse(&mut self) -> ParseResult<Vec<Stmt>> {
        let mut statements = vec![];

        while self.peek()?.is_some() {
            statements.push(self.statement()?);
        }

        Ok(statements)
    }

    pub fn synchronize(&mut self) -> ParseResult<()> {
        let token = self.advance()?;

        if let Some(token) = token {
            if *token.kind() == TokenKind::Semicolon {
                return Ok(());
            }
        }

        while let Some(operator) = self.peek()? {
            match operator.kind() {
                TokenKind::Class
                | TokenKind::Fun
                | TokenKind::Var
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Print
                | TokenKind::Return => return Ok(()),
                _ => self.advance()?,
            };
        }

        Ok(())
    }

    pub fn statement(&mut self) -> ParseResult<Stmt> {
        if let Some(operator) = self.peek()?.cloned() {
            let (stmt, error_message) = match operator.kind() {
                TokenKind::Print => (self.print_statement()?, "Expected ';' after expression."),
                _ => (
                    self.expression_statement()?,
                    "Expected ';' after expression.",
                ),
            };

            self.consume(TokenKind::Semicolon, Some(error_message.into()))?;
            Ok(stmt)
        } else {
            Err(SyntaxError::MissingExpression(Location::EOF))?
        }
    }

    pub fn expression_statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.expression()?;
        Ok(Stmt::Expression(*expr))
    }

    pub fn print_statement(&mut self) -> ParseResult<Stmt> {
        self.advance()?;
        let expr = self.expression()?;
        Ok(Stmt::Print(*expr))
    }

    pub fn expression(&mut self) -> ParseResult<Box<Expr>> {
        self.equality()
    }

    fn equality(&mut self) -> ParseResult<Box<Expr>> {
        let mut expr = self.comparison()?;

        while let Some(operator) = self.peek()?.cloned() {
            match operator.kind() {
                TokenKind::BangEqual | TokenKind::EqualEqual => {
                    self.advance()?;
                    let right = self.comparison()?;

                    expr = Box::new(Expr::Binary(expr, operator.into(), right));
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    pub fn comparison(&mut self) -> ParseResult<Box<Expr>> {
        let mut expr = self.term()?;

        while let Some(operator) = self.peek()?.cloned() {
            match operator.kind() {
                TokenKind::Greater
                | TokenKind::GreaterEqual
                | TokenKind::Less
                | TokenKind::LessEqual => {
                    self.advance()?;
                    let right = self.term()?;

                    expr = Box::new(Expr::Binary(expr, operator.into(), right));
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> ParseResult<Box<Expr>> {
        let mut expr = self.factor()?;

        while let Some(operator) = self.peek()?.cloned() {
            match operator.kind() {
                TokenKind::Minus | TokenKind::Plus => {
                    self.advance()?;
                    let right = self.factor()?;

                    expr = Box::new(Expr::Binary(expr, operator.into(), right));
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> ParseResult<Box<Expr>> {
        let mut expr = self.unary()?;

        while let Some(operator) = self.peek()?.cloned() {
            match operator.kind() {
                TokenKind::Slash | TokenKind::Star => {
                    self.advance()?;
                    let right = self.unary()?;

                    expr = Box::new(Expr::Binary(expr, operator.into(), right));
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> ParseResult<Box<Expr>> {
        if let Some(operator) = self.peek()?.cloned() {
            match operator.kind() {
                TokenKind::Bang | TokenKind::Minus => {
                    self.advance()?;
                    let right = self.unary()?;

                    return Ok(Box::new(Expr::Unary(operator.into(), right)));
                }
                _ => {}
            }
        }

        self.primary()
    }

    fn primary(&mut self) -> ParseResult<Box<Expr>> {
        macro_rules! literal {
            ($token:expr) => {{
                self.advance()?;
                Ok(Box::new(Expr::Literal($token)))
            }};
        }

        if let Some(token) = self.peek()?.cloned() {
            match token.kind() {
                TokenKind::False => literal!(Literal::Bool(false, token.location())),
                TokenKind::True => literal!(Literal::Bool(true, token.location())),
                TokenKind::Nil => literal!(Literal::Nil(token.location())),
                TokenKind::Number(n) => literal!(Literal::Number(*n, token.location())),
                TokenKind::String(s) => literal!(Literal::String(s.clone(), token.location())),
                TokenKind::LeftParen => self.grouping(token),
                _ => Err(SyntaxError::UnexpectedExpression(
                    token.location(),
                    token.lexeme().into(),
                ))?,
            }
        } else {
            Err(SyntaxError::MissingExpression(Location::EOF))?
        }
    }

    fn grouping(&mut self, token: Token) -> ParseResult<Box<Expr>> {
        self.advance()?;
        let expr = self.expression()?;

        if let Some(operator) = self.peek()? {
            match operator.kind() {
                TokenKind::RightParen => {
                    self.advance()?;
                    Ok(Box::new(Expr::Grouping(expr)))
                }
                _ => Err(SyntaxError::MissingClosingParenthesis(token.location()))?,
            }
        } else {
            Err(SyntaxError::MissingClosingParenthesis(Location::EOF))?
        }
    }

    fn advance(&mut self) -> Result<Option<Token>, ScanningError> {
        match self.buffer.pop_front() {
            None => match self.scanner.next().transpose()? {
                Some(g) => Ok(Some(g)),
                None => Ok(None),
            },
            Some(s) => Ok(Some(s)),
        }
    }

    fn consume(&mut self, kind: TokenKind, message: Option<String>) -> ParseResult<()> {
        let error_message = message.unwrap_or_else(|| format!("Expected token {}", kind));

        if let Some(token) = self.peek()? {
            match token.kind() {
                token if token.variant_eq(&kind) => {
                    self.advance()?;
                    Ok(())
                }
                _ => Err(SyntaxError::ExpectedToken(token.location(), error_message))?,
            }
        } else {
            Err(SyntaxError::ExpectedToken(Location::EOF, error_message))?
        }
    }

    fn peek(&mut self) -> Result<Option<&Token>, ScanningError> {
        self.peek_many(1)
    }

    fn peek_many(&mut self, qty: usize) -> Result<Option<&Token>, ScanningError> {
        assert_ne!(0, qty);

        while self.buffer.len() < qty {
            if let Some(next) = self.scanner.next().transpose()? {
                self.buffer.push_back(next);
            } else {
                return Ok(None);
            }
        }

        Ok(self.buffer.get(qty - 1))
    }
}
