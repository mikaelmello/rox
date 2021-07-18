use crate::lexer::{
    location::Location,
    scan_result::ScanningError,
    scanner::TokenIter,
    token::{Token, TokenKind},
};
use std::{
    collections::VecDeque,
    io::{Read, Seek},
};

use self::{
    ast::Expr,
    parse_error::{ParseError, SyntaxError},
};

mod ast;
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

    pub fn synchronize(&mut self) -> Result<(), ParseError> {
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

    pub fn expression(&mut self) -> Result<Box<Expr>, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.comparison()?;

        while let Some(operator) = self.peek()?.cloned() {
            match operator.kind() {
                TokenKind::BangEqual | TokenKind::EqualEqual => {
                    self.advance()?;
                    let right = self.comparison()?;

                    expr = Box::new(Expr::Binary(expr, operator, right));
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    pub fn comparison(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.term()?;

        while let Some(operator) = self.peek()?.cloned() {
            match operator.kind() {
                TokenKind::Greater
                | TokenKind::GreaterEqual
                | TokenKind::Less
                | TokenKind::LessEqual => {
                    self.advance()?;
                    let right = self.term()?;

                    expr = Box::new(Expr::Binary(expr, operator, right));
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.factor()?;

        while let Some(operator) = self.peek()?.cloned() {
            match operator.kind() {
                TokenKind::Minus | TokenKind::Plus => {
                    self.advance()?;
                    let right = self.factor()?;

                    expr = Box::new(Expr::Binary(expr, operator, right));
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<Expr>, ParseError> {
        let mut expr = self.unary()?;

        while let Some(operator) = self.peek()?.cloned() {
            match operator.kind() {
                TokenKind::Slash | TokenKind::Star => {
                    self.advance()?;
                    let right = self.unary()?;

                    expr = Box::new(Expr::Binary(expr, operator, right));
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<Expr>, ParseError> {
        if let Some(operator) = self.peek()?.cloned() {
            match operator.kind() {
                TokenKind::Bang | TokenKind::Minus => {
                    self.advance()?;
                    let right = self.unary()?;

                    return Ok(Box::new(Expr::Unary(operator, right)));
                }
                _ => {}
            }
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Box<Expr>, ParseError> {
        macro_rules! literal {
            ($token:expr) => {{
                self.advance()?;
                Ok(Box::new(Expr::Literal($token)))
            }};
        }

        if let Some(token) = self.peek()?.cloned() {
            println!("{:?} what is happening", token);
            match token.kind() {
                TokenKind::False => literal!(token),
                TokenKind::True => literal!(token),
                TokenKind::Nil => literal!(token),
                TokenKind::Number(_) => literal!(token),
                TokenKind::String(_) => literal!(token),
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

    fn grouping(&mut self, token: Token) -> Result<Box<Expr>, ParseError> {
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
