use super::{
    config::reserved_token,
    lexical_error::LexicalError,
    location::Location,
    token::{Token, TokenType},
};
use crate::error::RoxError;
use std::io::{self, BufReader, Bytes, Read, Seek};
use unicode_reader::{CodePoints, Graphemes};

pub struct Scanner<T: Read + Seek> {
    inp: Graphemes<CodePoints<Bytes<BufReader<T>>>>,
    loc: Location,
    buf: Option<String>,
    cur: String,
    errors: Vec<LexicalError>,
    invalid_string: Option<(String, Location)>,
}

impl<T: Read + Seek> Scanner<T> {
    pub fn from(source: T) -> Result<Self, RoxError> {
        let reader = BufReader::new(source);
        let graphemes = Graphemes::from(reader);

        Ok(Self {
            inp: graphemes,
            loc: Location::default(),
            buf: None,
            cur: String::new(),
            errors: vec![],
            invalid_string: None,
        })
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, RoxError> {
        let mut tokens = vec![];
        while let Some(_) = self.peek()? {
            match self.next_token()? {
                Some(t) => tokens.push(t),
                None => {}
            }
        }

        tokens.push(Token::new(TokenType::Eof, "", self.loc));

        Ok(tokens)
    }

    fn error(&mut self, error: LexicalError) {
        self.errors.push(error);
    }

    fn invalid_grapheme(&mut self, grapheme: &str, loc: Location) {
        let invalid = match self.invalid_string.take() {
            Some((mut string, loc)) => {
                string.push_str(grapheme);
                (string, loc)
            }
            None => (String::from(grapheme), loc),
        };

        self.invalid_string = Some(invalid);
    }

    fn build_token(&self, r#type: TokenType, loc: Location) -> Token {
        Token::new(r#type, &self.cur, loc)
    }

    fn next_token(&mut self) -> Result<Option<Token>, RoxError> {
        self.cur = String::new();
        let loc = self.loc;

        let grapheme = match self.advance()? {
            Some(s) => s,
            None => return Ok(None),
        };

        macro_rules! token {
            ($type:expr) => {
                Ok(Some(self.build_token($type, loc)))
            };
        }

        match &grapheme[..] {
            "(" => token!(TokenType::LeftParen),
            ")" => token!(TokenType::RightParen),
            "{" => token!(TokenType::LeftBrace),
            "}" => token!(TokenType::RightBrace),
            "," => token!(TokenType::Comma),
            "." => token!(TokenType::Dot),
            "-" => token!(TokenType::Minus),
            "+" => token!(TokenType::Plus),
            ";" => token!(TokenType::Semicolon),
            "*" => token!(TokenType::Star),

            // two-char tokens
            "!" if self.peek_match("=")? => token!(TokenType::BangEqual),
            "!" => token!(TokenType::Bang),

            "=" if self.peek_match("=")? => token!(TokenType::EqualEqual),
            "=" => token!(TokenType::Equal),

            "<" if self.peek_match("=")? => token!(TokenType::LessEqual),
            "<" => token!(TokenType::Less),

            ">" if self.peek_match("=")? => token!(TokenType::GreaterEqual),
            ">" => token!(TokenType::Greater),

            "/" if self.peek_match("/")? => {
                while self.peek()? != Some("\n") {
                    self.advance()?;
                }
                Ok(None)
            }
            "/" => token!(TokenType::Slash),

            "\n" => Ok(None),

            "\"" => return Ok(self.string(loc)?),
            g if Self::is_digit(g) => Ok(self.number(loc)?),
            g if Self::is_alpha(g) => Ok(Some(self.identifier(loc)?)),
            " " | "\r" | "\t" => Ok(None),
            g => {
                self.invalid_grapheme(g, loc);
                Ok(None)
            }
        }
    }

    fn identifier(&mut self, start_loc: Location) -> Result<Token, io::Error> {
        while let Some(c) = self.peek()? {
            if Self::is_alpha_numeric(c) {
                self.advance()?;
            } else {
                break;
            }
        }

        if let Some(reserved) = reserved_token(&self.cur) {
            Ok(self.build_token(reserved, start_loc))
        } else {
            Ok(self.build_token(TokenType::Identifier(self.cur.clone()), start_loc))
        }
    }

    fn number(&mut self, start_loc: Location) -> Result<Option<Token>, io::Error> {
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

        match self.cur.parse::<f64>() {
            Ok(literal) => Ok(Some(
                self.build_token(TokenType::Number(literal), start_loc),
            )),
            Err(_) => {
                self.error(LexicalError::InvalidNumberLiteral(
                    self.cur.clone(),
                    start_loc,
                ));
                Ok(None)
            }
        }
    }

    fn string(&mut self, start_loc: Location) -> Result<Option<Token>, io::Error> {
        let mut literal = String::new();

        while let Some(c) = self.peek()? {
            if c == "\"" {
                break;
            }

            literal.push_str(c);

            self.advance()?;
        }

        if self.peek()? != Some("\"") {
            self.error(LexicalError::UnterminatedString(start_loc));
            return Ok(None);
        }

        self.advance()?;

        Ok(Some(
            self.build_token(TokenType::String(literal), start_loc),
        ))
    }

    fn advance(&mut self) -> Result<Option<String>, io::Error> {
        let res = match self.buf.take() {
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
        if self.buf.is_none() {
            self.buf = self.next()?;
        }

        Ok(self.buf.as_deref())
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
