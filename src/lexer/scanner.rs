use super::{
    config::reserved_token,
    lexical_error::LexicalError,
    location::Location,
    token::{Token, TokenKind},
};
use crate::{error::RoxError, lexer::scan_result::ScanResult};
use std::{
    collections::VecDeque,
    io::{self, BufReader, Bytes, Read, Seek},
};
use unicode_reader::{CodePoints, Graphemes};

pub struct Scanner<T: Read + Seek> {
    inp: Graphemes<CodePoints<Bytes<BufReader<T>>>>,
    loc: Location,
    buf: VecDeque<String>,
    cur: String,
}

impl<T: Read + Seek> Scanner<T> {
    pub fn from(source: T) -> Result<Self, RoxError> {
        let reader = BufReader::new(source);
        let graphemes = Graphemes::from(reader);

        Ok(Self {
            inp: graphemes,
            loc: Location::default(),
            buf: VecDeque::new(),
            cur: String::new(),
        })
    }

    pub fn scan_tokens(mut self) -> Result<(Vec<Token>, Vec<LexicalError>), RoxError> {
        let mut tokens = vec![];
        let mut errors = vec![];

        loop {
            match self.next_token() {
                Ok(None) => break,
                Ok(Some(token)) => tokens.push(token),
                Err(LexicalError::IO(err)) => return Err(RoxError::IO(err)),
                Err(err) => errors.push(err),
            };
        }

        Ok((tokens, errors))
    }

    fn build_token(&self, kind: TokenKind, loc: Location) -> Token {
        Token::new(kind, &self.cur, loc)
    }

    fn next_token(&mut self) -> ScanResult {
        loop {
            self.cur = String::new();
            let loc = self.loc;

            macro_rules! token {
                ($kind:expr) => {
                    return Ok(Some(self.build_token($kind, loc)));
                };
            }

            let grapheme = match self.advance()? {
                Some(g) => g,
                None => return Ok(None),
            };

            match &grapheme[..] {
                "(" => token!(TokenKind::LeftParen),
                ")" => token!(TokenKind::RightParen),
                "{" => token!(TokenKind::LeftBrace),
                "}" => token!(TokenKind::RightBrace),
                "," => token!(TokenKind::Comma),
                "." => token!(TokenKind::Dot),
                "-" => token!(TokenKind::Minus),
                "+" => token!(TokenKind::Plus),
                ";" => token!(TokenKind::Semicolon),
                "*" => token!(TokenKind::Star),

                // two-char tokens
                "!" if self.peek_match("=")? => token!(TokenKind::BangEqual),
                "!" => token!(TokenKind::Bang),

                "=" if self.peek_match("=")? => token!(TokenKind::EqualEqual),
                "=" => token!(TokenKind::Equal),

                "<" if self.peek_match("=")? => token!(TokenKind::LessEqual),
                "<" => token!(TokenKind::Less),

                ">" if self.peek_match("=")? => token!(TokenKind::GreaterEqual),
                ">" => token!(TokenKind::Greater),

                "/" if self.peek_match("/")? => while self.peek_match("\n")? {},
                "/" => token!(TokenKind::Slash),

                "\n" => {}
                " " | "\r" | "\t" => {}

                "\"" => return self.string(loc),
                g if Self::is_digit(g) => return self.number(loc),
                g if Self::is_alpha(g) => return self.identifier(loc),
                _ => return Err(LexicalError::InvalidLexeme(grapheme, loc)),
            }
        }
    }

    fn identifier(&mut self, start_loc: Location) -> ScanResult {
        while let Some(c) = self.peek()? {
            if Self::is_alpha_numeric(c) {
                self.advance()?;
            } else {
                break;
            }
        }

        if let Some(reserved) = reserved_token(&self.cur) {
            Ok(Some(self.build_token(reserved, start_loc)))
        } else {
            Ok(Some(self.build_token(
                TokenKind::Identifier(self.cur.clone()),
                start_loc,
            )))
        }
    }

    fn number(&mut self, start_loc: Location) -> ScanResult {
        while let Some(c) = self.peek()? {
            if Self::is_digit(c) {
                self.advance()?;
            } else {
                break;
            }
        }

        if self.peek()? == Some(".") {
            self.advance()?;

            while let Some(c) = self.peek()? {
                match c {
                    c if Self::is_digit(c) => self.advance()?,
                    _ => break,
                };
            }
        }

        self.cur
            .parse::<f64>()
            .map(|l| Some(self.build_token(TokenKind::Number(l), start_loc)))
            .map_err(|_| LexicalError::InvalidNumberLiteral(self.cur.clone(), start_loc))
    }

    fn string(&mut self, start_loc: Location) -> ScanResult {
        let mut literal = String::new();

        while let Some(c) = self.peek()? {
            if c == "\"" {
                break;
            }

            literal.push_str(c);

            self.advance()?;
        }

        if self.peek()? != Some("\"") {
            return Err(LexicalError::UnterminatedString(start_loc));
        }

        self.advance()?;

        Ok(Some(
            self.build_token(TokenKind::String(literal), start_loc),
        ))
    }

    fn advance(&mut self) -> Result<Option<String>, io::Error> {
        let res = match self.buf.pop_front() {
            None => match self.next()? {
                Some(g) => {
                    self.cur.push_str(&g);
                    Ok(Some(g))
                }
                None => Ok(None),
            },
            Some(s) => {
                self.cur.push_str(&s);
                Ok(Some(s))
            }
        };

        if let Ok(Some(s)) = &res {
            if "\n" == s {
                self.loc.lf();
                self.loc.cr();
            } else {
                self.loc.next();
            }
        }

        res
    }

    fn peek(&mut self) -> Result<Option<&str>, io::Error> {
        self.peek_many(1)
    }

    fn peek_many(&mut self, qty: usize) -> Result<Option<&str>, io::Error> {
        assert_ne!(0, qty);

        while self.buf.len() < qty {
            if let Some(next) = self.next()? {
                self.buf.push_back(next);
            } else {
                return Ok(None);
            }
        }

        Ok(self.buf.get(qty - 1).map(|s| &s[..]))
    }

    fn peek_match(&mut self, expected: &str) -> Result<bool, io::Error> {
        if self.peek()? == Some(expected) {
            self.advance()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn is_alpha_numeric(g: &str) -> bool {
        Self::is_alpha(g) || Self::is_digit(g)
    }

    fn is_alpha(g: &str) -> bool {
        g.len() == 1
            && ((g.chars().nth(0).unwrap() >= 'a' && g.chars().nth(0).unwrap() <= 'z')
                || (g.chars().nth(0).unwrap() >= 'A' && g.chars().nth(0).unwrap() <= 'Z'))
    }

    fn is_digit(g: &str) -> bool {
        g.len() == 1 && g.chars().nth(0).unwrap() >= '0' && g.chars().nth(0).unwrap() <= '9'
    }

    fn next(&mut self) -> Result<Option<String>, io::Error> {
        self.inp.next().transpose()
    }
}
pub struct TokenIter<T: Read + Seek> {
    scanner: Scanner<T>,
}

impl<T: Read + Seek> Scanner<T> {
    pub fn into_iter(self) -> TokenIter<T> {
        TokenIter { scanner: self }
    }
}

impl<T: Read + Seek> Iterator for TokenIter<T> {
    type Item = Result<Token, LexicalError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.scanner.next_token().transpose()
    }
}
