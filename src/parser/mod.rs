use std::cmp::max;

use self::{
    ast::{Expr, Stmt},
    parse_error::{ParseErrorKind, ParseResult},
};
use crate::{
    location::Location,
    parser::{
        ast::{Literal, LiteralKind},
        parse_error::ParseError,
    },
    scanner::{scanner::TokenIter, Scanner, Token, TokenKind},
};

pub mod ast;
mod parse_error;

pub struct Parser<'sourcecode> {
    scanner: TokenIter<'sourcecode>,
    buffer: Option<Token<'sourcecode>>,
}

impl<'sourcecode> Parser<'sourcecode> {
    pub fn new(scanner: Scanner<'sourcecode>) -> Self {
        Self {
            scanner: scanner.into_iter(),
            buffer: None,
        }
    }

    pub fn parse(&mut self) -> ParseResult<Vec<Stmt>> {
        let mut statements = vec![];

        while self.peek().is_some() {
            statements.push(self.statement()?);
        }

        Ok(statements)
    }

    pub fn synchronize(&mut self) -> ParseResult<()> {
        let token = self.advance();

        if let Some(token) = token {
            if token.kind() == TokenKind::Semicolon {
                return Ok(());
            }
        }

        while let Some(operator) = self.peek() {
            match operator.kind() {
                TokenKind::Class
                | TokenKind::Fun
                | TokenKind::Var
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Print
                | TokenKind::Return => return Ok(()),
                _ => self.advance(),
            };
        }

        Ok(())
    }

    pub fn statement(&mut self) -> ParseResult<Stmt> {
        if let Some(operator) = self.peek() {
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
            Err(ParseError::new(
                ParseErrorKind::MissingExpression,
                Location::EOF,
            ))?
        }
    }

    pub fn expression_statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.expression()?;
        Ok(Stmt::Expression(*expr))
    }

    pub fn print_statement(&mut self) -> ParseResult<Stmt> {
        self.advance();
        let expr = self.expression()?;
        Ok(Stmt::Print(*expr))
    }

    pub fn expression(&mut self) -> ParseResult<Box<Expr>> {
        self.equality()
    }

    fn equality(&mut self) -> ParseResult<Box<Expr>> {
        let mut expr = self.comparison()?;

        while let Some(operator) = self.peek() {
            match operator.kind() {
                TokenKind::BangEqual | TokenKind::EqualEqual => {
                    let op = operator.into();

                    self.advance();
                    let right = self.comparison()?;

                    expr = Box::new(Expr::Binary(expr, op, right));
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    pub fn comparison(&mut self) -> ParseResult<Box<Expr>> {
        let mut expr = self.term()?;

        while let Some(operator) = self.peek() {
            match operator.kind() {
                TokenKind::Greater
                | TokenKind::GreaterEqual
                | TokenKind::Less
                | TokenKind::LessEqual => {
                    let op = operator.into();

                    self.advance();
                    let right = self.term()?;

                    expr = Box::new(Expr::Binary(expr, op, right));
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> ParseResult<Box<Expr>> {
        let mut expr = self.factor()?;

        while let Some(operator) = self.peek() {
            match operator.kind() {
                TokenKind::Minus | TokenKind::Plus => {
                    let op = operator.into();

                    self.advance();
                    let right = self.factor()?;

                    expr = Box::new(Expr::Binary(expr, op, right));
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> ParseResult<Box<Expr>> {
        let mut expr = self.unary()?;

        while let Some(operator) = self.peek() {
            match operator.kind() {
                TokenKind::Slash | TokenKind::Star => {
                    let op = operator.into();

                    self.advance();
                    let right = self.unary()?;

                    expr = Box::new(Expr::Binary(expr, op, right));
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> ParseResult<Box<Expr>> {
        if let Some(operator) = self.peek() {
            match operator.kind() {
                TokenKind::Bang | TokenKind::Minus => {
                    let op = operator.into();

                    self.advance();
                    let right = self.unary()?;

                    return Ok(Box::new(Expr::Unary(op, right)));
                }
                _ => {}
            }
        }

        self.primary()
    }

    fn primary(&mut self) -> ParseResult<Box<Expr>> {
        if let Some(token) = self.peek() {
            macro_rules! literal {
                ($kind:expr) => {{
                    let loc = token.location();

                    self.advance();
                    let literal = Literal::new($kind, loc);
                    Ok(Box::new(Expr::Literal(literal)))
                }};
            }

            match token.kind() {
                TokenKind::False => literal!(LiteralKind::Bool(false)),
                TokenKind::True => literal!(LiteralKind::Bool(true)),
                TokenKind::Nil => literal!(LiteralKind::Nil),

                TokenKind::String => {
                    let end = max(1, token.lexeme().len().saturating_sub(1));
                    let literal = &token.lexeme()[1..end];

                    let literal = LiteralKind::String(literal.into());
                    literal!(literal)
                }

                TokenKind::Number => {
                    let number = match token.lexeme().parse::<f64>() {
                        Ok(number) => number,
                        Err(_) => Err(ParseError::new(
                            ParseErrorKind::InvalidNumberLiteral(token.lexeme().into()),
                            token.location(),
                        ))?,
                    };

                    literal!(LiteralKind::Number(number))
                }

                TokenKind::LeftParen => self.grouping(),

                _ => Err(ParseError::new(
                    ParseErrorKind::UnexpectedExpression(token.lexeme().into()),
                    token.location(),
                ))?,
            }
        } else {
            Err(ParseError::new(
                ParseErrorKind::MissingExpression,
                Location::EOF,
            ))?
        }
    }

    fn grouping(&mut self) -> ParseResult<Box<Expr>> {
        self.advance();
        let expr = self.expression()?;

        if let Some(token) = self.peek() {
            match token.kind() {
                TokenKind::RightParen => {
                    self.advance();
                    Ok(Box::new(Expr::Grouping(expr)))
                }
                _ => Err(ParseError::new(
                    ParseErrorKind::MissingClosingParenthesis,
                    token.location(),
                ))?,
            }
        } else {
            Err(ParseError::new(
                ParseErrorKind::MissingClosingParenthesis,
                Location::EOF,
            ))?
        }
    }

    #[inline]
    fn advance(&mut self) -> Option<Token> {
        match self.buffer.take() {
            None => self.scanner.next(),
            Some(s) => Some(s),
        }
    }

    fn consume(&mut self, kind: TokenKind, message: Option<String>) -> ParseResult<()> {
        let error_message = message.unwrap_or_else(|| format!("Expected token {}", kind));

        if let Some(token) = self.peek() {
            if token.kind() == kind {
                self.advance();
                Ok(())
            } else {
                Err(ParseError::new(
                    ParseErrorKind::ExpectedToken(error_message),
                    token.location(),
                ))?
            }
        } else {
            Err(ParseError::new(
                ParseErrorKind::ExpectedToken(error_message),
                Location::EOF,
            ))?
        }
    }

    #[inline]
    fn peek(&mut self) -> Option<Token> {
        if let Some(token) = self.buffer {
            return Some(token);
        }

        self.buffer = self.scanner.next();

        self.buffer
    }
}
