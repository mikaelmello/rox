use std::{
    collections::VecDeque,
    io::{Read, Seek},
};

use crate::{
    ast::Expr,
    lexer::{
        lexical_error::LexicalError,
        scanner::TokenIter,
        token::{Token, TokenType},
    },
};

pub struct Parser<T: Read + Seek> {
    scanner: TokenIter<T>,
    buffer: VecDeque<Token>,
}

impl<T: Read + Seek> Parser<T> {
    pub fn expression(&mut self) -> Result<Box<Expr>, LexicalError> {
        self.equality()
    }

    pub fn equality(&mut self) -> Result<Box<Expr>, LexicalError> {
        let expr = self.comparison()?;

        while let Some(operator) = self.r#match(&[TokenType::BangEqual, TokenType::EqualEqual])? {
            let right = self.term()?;

            return Ok(Box::new(Expr::Binary(expr, operator, right)));
        }

        Ok(expr)
    }

    pub fn comparison(&mut self) -> Result<Box<Expr>, LexicalError> {
        let expr = self.term()?;

        while let Some(operator) = self.r#match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ])? {
            let right = self.term()?;

            return Ok(Box::new(Expr::Binary(expr, operator, right)));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<Expr>, LexicalError> {
        let expr = self.factor()?;

        while let Some(operator) = self.r#match(&[TokenType::Minus, TokenType::Plus])? {
            let right = self.factor()?;

            return Ok(Box::new(Expr::Binary(expr, operator, right)));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<Expr>, LexicalError> {
        let expr = self.unary()?;

        while let Some(operator) = self.r#match(&[TokenType::Slash, TokenType::Star])? {
            let right = self.unary()?;

            return Ok(Box::new(Expr::Binary(expr, operator, right)));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<Expr>, LexicalError> {
        if let Some(operator) = self.r#match(&[TokenType::Bang, TokenType::Minus])? {
            let right = self.unary()?;

            return Ok(Box::new(Expr::Unary(operator, right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Box<Expr>, LexicalError> {
        if let Some(token) = self.peek()?.cloned() {
            match token.r#type() {
                TokenType::False => Ok(Box::new(Expr::Literal(token))),
                TokenType::True => Ok(Box::new(Expr::Literal(token))),
                TokenType::Nil => Ok(Box::new(Expr::Literal(token))),
                TokenType::Number(_) => Ok(Box::new(Expr::Literal(token))),
                TokenType::String(_) => Ok(Box::new(Expr::Literal(token))),
                TokenType::LeftParen => {
                    let expr = self.expression()?;
                    self.consume(&TokenType::RightParen, "Expect ')' after expression.")?;

                    Ok(Box::new(Expr::Grouping(expr)))
                }
                _ => Err(LexicalError::Unknown),
            }
        } else {
            Err(LexicalError::Unknown)
        }
    }

    fn consume(&mut self, kind: &TokenType, msg: &str) -> Result<Option<Token>, LexicalError> {
        if self.check(&kind)? {
            self.advance()
        } else {
            Err(LexicalError::Unknown)
        }
    }

    fn r#match(&mut self, types: &[TokenType]) -> Result<Option<Token>, LexicalError> {
        for kind in types {
            if self.check(kind)? {
                return self.advance();
            }
        }

        Ok(None)
    }

    fn check(&mut self, kind: &TokenType) -> Result<bool, LexicalError> {
        Ok(self
            .peek()?
            .map(|t| t.r#type())
            .filter(|k| *k == kind)
            .is_some())
    }

    fn advance(&mut self) -> Result<Option<Token>, LexicalError> {
        match self.buffer.pop_front() {
            None => match self.scanner.next().transpose()? {
                Some(g) => Ok(Some(g)),
                None => Ok(None),
            },
            Some(s) => Ok(Some(s)),
        }
    }

    fn peek(&mut self) -> Result<Option<&Token>, LexicalError> {
        self.peek_many(1)
    }

    fn peek_many(&mut self, qty: usize) -> Result<Option<&Token>, LexicalError> {
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
