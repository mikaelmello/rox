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

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::lexer::token::{Token, TokenType};

    use super::Location;
    use super::Scanner;

    macro_rules! token {
        ($type:expr,$lexeme:expr) => {
            token!($type, $lexeme, Location::default())
        };

        ($type:expr,$lexeme:expr,$loc:expr) => {
            Token::new($type, $lexeme, Location::from($loc))
        };
    }

    macro_rules! test {
        ($name:ident,$input:expr,$output:expr) => {
            #[test]
            fn $name() {
                let cursor = Cursor::new($input);

                let tokens = Scanner::from(cursor)
                    .and_then(Scanner::scan_tokens)
                    .unwrap();

                for (idx, t) in tokens.iter().enumerate() {
                    let ot = &$output[idx];
                    assert_eq!(ot, t);
                }
            }
        };
    }

    test!(
        single_chars,
        "(){},-.;+*",
        vec![
            token!(TokenType::LeftParen, "(", (0, 0)),
            token!(TokenType::RightParen, ")", (0, 1)),
            token!(TokenType::LeftBrace, "{", (0, 2)),
            token!(TokenType::RightBrace, "}", (0, 3)),
            token!(TokenType::Comma, ",", (0, 4)),
            token!(TokenType::Minus, "-", (0, 5)),
            token!(TokenType::Dot, ".", (0, 6)),
            token!(TokenType::Semicolon, ";", (0, 7)),
            token!(TokenType::Plus, "+", (0, 8)),
            token!(TokenType::Star, "*", (0, 9)),
            token!(TokenType::Eof, "", (0, 10)),
        ]
    );

    test!(
        single_chars_repeated,
        "(){}**--,-.;+*",
        vec![
            token!(TokenType::LeftParen, "(", (0, 0)),
            token!(TokenType::RightParen, ")", (0, 1)),
            token!(TokenType::LeftBrace, "{", (0, 2)),
            token!(TokenType::RightBrace, "}", (0, 3)),
            token!(TokenType::Star, "*", (0, 4)),
            token!(TokenType::Star, "*", (0, 5)),
            token!(TokenType::Minus, "-", (0, 6)),
            token!(TokenType::Minus, "-", (0, 7)),
            token!(TokenType::Comma, ",", (0, 8)),
            token!(TokenType::Minus, "-", (0, 9)),
            token!(TokenType::Dot, ".", (0, 10)),
            token!(TokenType::Semicolon, ";", (0, 11)),
            token!(TokenType::Plus, "+", (0, 12)),
            token!(TokenType::Star, "*", (0, 13)),
            token!(TokenType::Eof, "", (0, 14)),
        ]
    );

    test!(
        strings,
        "\"singleword\"",
        vec![
            token!(
                TokenType::String("singleword".into()),
                "\"singleword\"",
                (0, 0)
            ),
            token!(TokenType::Eof, "", (0, 12)),
        ]
    );

    test!(
        strings_and_chars_and_lines,
        "var x = \"singleword\"\n{}",
        vec![
            token!(TokenType::Var, "var", (0, 0)),
            token!(TokenType::Identifier("x".into()), "x", (0, 4)),
            token!(TokenType::Equal, "=", (0, 6)),
            token!(
                TokenType::String("singleword".into()),
                "\"singleword\"",
                (0, 8)
            ),
            token!(TokenType::LeftBrace, "{", (1, 0)),
            token!(TokenType::RightBrace, "}", (1, 1)),
            token!(TokenType::Eof, "", (1, 2)),
        ]
    );

    test!(
        strings_and_chars_and_lines_extended,
        "var x = \"singleword\";\nvar y = 2 + 3.;\nif (y >= 5.42 or y < 0.0000) quit();",
        vec![
            token!(TokenType::Var, "var", (0, 0)),
            token!(TokenType::Identifier("x".into()), "x", (0, 4)),
            token!(TokenType::Equal, "=", (0, 6)),
            token!(
                TokenType::String("singleword".into()),
                "\"singleword\"",
                (0, 8)
            ),
            token!(TokenType::Semicolon, ";", (0, 20)),
            token!(TokenType::Var, "var", (1, 0)),
            token!(TokenType::Identifier("y".into()), "y", (1, 4)),
            token!(TokenType::Equal, "=", (1, 6)),
            token!(TokenType::Number(2f64), "2", (1, 8)),
            token!(TokenType::Plus, "+", (1, 10)),
            token!(TokenType::Number(3f64), "3.", (1, 12)),
            token!(TokenType::Semicolon, ";", (1, 14)),
            token!(TokenType::If, "if", (2, 0)),
            token!(TokenType::LeftParen, "(", (2, 3)),
            token!(TokenType::Identifier("y".into()), "y", (2, 4)),
            token!(TokenType::GreaterEqual, ">=", (2, 6)),
            token!(TokenType::Number(5.42f64), "5.42", (2, 9)),
            token!(TokenType::Or, "or", (2, 14)),
            token!(TokenType::Identifier("y".into()), "y", (2, 17)),
            token!(TokenType::Less, "<", (2, 19)),
            token!(TokenType::Number(0f64), "0.0000", (2, 21)),
            token!(TokenType::RightParen, ")", (2, 27)),
            token!(TokenType::Identifier("quit".into()), "quit", (2, 29)),
            token!(TokenType::LeftParen, "(", (2, 33)),
            token!(TokenType::RightParen, ")", (2, 34)),
            token!(TokenType::Semicolon, ";", (2, 35)),
            token!(TokenType::Eof, "", (2, 36)),
        ]
    );
}
