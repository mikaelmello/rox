use crate::{
    ast::Expr,
    lexer::{
        lexical_error::LexicalError,
        scanner::TokenIter,
        token::{Token, TokenKind},
    },
};
use std::{
    collections::VecDeque,
    io::{Read, Seek},
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

        while let Some(operator) = self.r#match(&[TokenKind::BangEqual, TokenKind::EqualEqual])? {
            let right = self.term()?;

            return Ok(Box::new(Expr::Binary(expr, operator, right)));
        }

        Ok(expr)
    }

    pub fn comparison(&mut self) -> Result<Box<Expr>, LexicalError> {
        let expr = self.term()?;

        while let Some(operator) = self.r#match(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ])? {
            let right = self.term()?;

            return Ok(Box::new(Expr::Binary(expr, operator, right)));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<Expr>, LexicalError> {
        let expr = self.factor()?;

        while let Some(operator) = self.r#match(&[TokenKind::Minus, TokenKind::Plus])? {
            let right = self.factor()?;

            return Ok(Box::new(Expr::Binary(expr, operator, right)));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<Expr>, LexicalError> {
        let expr = self.unary()?;

        while let Some(operator) = self.r#match(&[TokenKind::Slash, TokenKind::Star])? {
            let right = self.unary()?;

            return Ok(Box::new(Expr::Binary(expr, operator, right)));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<Expr>, LexicalError> {
        if let Some(operator) = self.r#match(&[TokenKind::Bang, TokenKind::Minus])? {
            let right = self.unary()?;

            return Ok(Box::new(Expr::Unary(operator, right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Box<Expr>, LexicalError> {
        if let Some(token) = self.peek()?.cloned() {
            match token.kind() {
                TokenKind::False => Ok(Box::new(Expr::Literal(token))),
                TokenKind::True => Ok(Box::new(Expr::Literal(token))),
                TokenKind::Nil => Ok(Box::new(Expr::Literal(token))),
                TokenKind::Number(_) => Ok(Box::new(Expr::Literal(token))),
                TokenKind::String(_) => Ok(Box::new(Expr::Literal(token))),
                TokenKind::LeftParen => {
                    let expr = self.expression()?;
                    self.consume(&TokenKind::RightParen, "Expect ')' after expression.")?;

                    Ok(Box::new(Expr::Grouping(expr)))
                }
                _ => Err(LexicalError::Unknown),
            }
        } else {
            Err(LexicalError::Unknown)
        }
    }

    fn consume(&mut self, kind: &TokenKind, msg: &str) -> Result<Option<Token>, LexicalError> {
        if self.check(&kind)? {
            self.advance()
        } else {
            Err(LexicalError::Unknown)
        }
    }

    fn r#match(&mut self, kinds: &[TokenKind]) -> Result<Option<Token>, LexicalError> {
        for kind in kinds {
            if self.check(kind)? {
                return self.advance();
            }
        }

        Ok(None)
    }

    fn check(&mut self, kind: &TokenKind) -> Result<bool, LexicalError> {
        Ok(self
            .peek()?
            .map(|t| t.kind())
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
