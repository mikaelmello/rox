use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Bytes, Read, Seek},
    sync::{Arc, RwLock},
};

use unicode_reader::{CodePoints, Graphemes};

use crate::error::RoxError;

use super::token::{Token, TokenType};

enum CodeSource {
    File(File),
    String(String),
}

fn reserved_token(lexeme: &str) -> Option<TokenType> {
    match lexeme {
        "and" => Some(TokenType::And),
        "class" => Some(TokenType::Class),
        "else" => Some(TokenType::Else),
        "false" => Some(TokenType::False),
        "for" => Some(TokenType::For),
        "fun" => Some(TokenType::Fun),
        "if" => Some(TokenType::If),
        "nil" => Some(TokenType::Nil),
        "or" => Some(TokenType::Or),
        "print" => Some(TokenType::Print),
        "return" => Some(TokenType::Return),
        "super" => Some(TokenType::Super),
        "this" => Some(TokenType::This),
        "true" => Some(TokenType::True),
        "var" => Some(TokenType::Var),
        "while" => Some(TokenType::While),
        _ => None,
    }
}

pub struct Scanner<T: Read + Seek> {
    graphemes: Graphemes<CodePoints<Bytes<BufReader<T>>>>,
    line: usize,
    buffer: Option<String>,
    cur: String,
}

impl<T: Read + Seek> Scanner<T> {
    pub fn from(source: T) -> Result<Self, RoxError> {
        let reader = BufReader::new(source);
        let graphemes = Graphemes::from(reader);

        Ok(Self {
            graphemes,
            line: 0,
            buffer: None,
            cur: String::new(),
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

        Ok(tokens)
    }

    fn build_token(&self, r#type: TokenType) -> Token {
        Token::new(r#type, self.cur.clone(), self.line)
    }

    fn next_token(&mut self) -> Result<Option<Token>, RoxError> {
        self.cur = String::new();

        let grapheme = match self.advance()? {
            Some(s) => s,
            None => return Ok(None),
        };

        macro_rules! token {
            ($type:expr) => {
                Ok(Some(self.build_token($type)))
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

            "\n" => {
                self.line = self.line.saturating_add(1);
                Ok(None)
            }

            "\"" => return Ok(Some(self.string()?)),
            g if Self::is_digit(g) => Ok(Some(self.number()?)),
            g if Self::is_alpha(g) => Ok(Some(self.identifier()?)),
            " " | "\r" | "\t" => Ok(None),
            _ => Err(RoxError::SyntaxError(
                self.line,
                format!("Invalid character {}", grapheme),
            )),
        }
    }

    fn identifier(&mut self) -> Result<Token, RoxError> {
        while let Some(c) = self.peek()? {
            if Self::is_alpha_numeric(c) {
                self.advance()?;
            } else {
                break;
            }
        }

        if let Some(reserved) = reserved_token(&self.cur) {
            Ok(self.build_token(reserved))
        } else {
            Ok(self.build_token(TokenType::Identifier(self.cur.clone())))
        }
    }

    fn number(&mut self) -> Result<Token, RoxError> {
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

        let literal = self
            .cur
            .parse::<f64>()
            .map_err(|_| RoxError::SyntaxError(self.line, "Invalid number literal".to_string()))?;

        Ok(self.build_token(TokenType::Number(literal)))
    }

    fn string(&mut self) -> Result<Token, RoxError> {
        let initial_line = self.line;
        let mut cur_line = initial_line;
        let mut literal = String::new();

        while let Some(c) = self.peek()? {
            if c == "\"" {
                break;
            }

            if c == "\n" {
                cur_line = cur_line.saturating_add(1);
            }

            literal.push_str(c);

            self.advance()?;
        }

        if self.peek()? != Some("\"") {
            return Err(RoxError::SyntaxError(
                initial_line,
                "Unterminated string".to_string(),
            ));
        }

        self.line = cur_line;
        self.advance()?;

        Ok(self.build_token(TokenType::String(literal)))
    }

    fn advance(&mut self) -> Result<Option<String>, RoxError> {
        match self.buffer.take() {
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
        }
    }

    fn peek(&mut self) -> Result<Option<&str>, RoxError> {
        if self.buffer.is_none() {
            self.buffer = self.next()?;
        }

        Ok(self.buffer.as_deref())
    }

    fn peek_match(&mut self, expected: &str) -> Result<bool, RoxError> {
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

    fn next(&mut self) -> Result<Option<String>, RoxError> {
        self.graphemes.next().transpose().map_err(RoxError::from)
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::lexer::token::{Token, TokenType};

    use super::Scanner;

    macro_rules! token {
        ($type:expr,$lexeme:expr) => {
            token!($type, $lexeme, 0)
        };

        ($type:expr,$lexeme:expr,$line:expr) => {
            Token::new($type, String::from($lexeme), $line)
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

                assert_eq!($output, tokens);
            }
        };
    }

    test!(
        single_chars,
        "(){},-.;+*",
        vec![
            token!(TokenType::LeftParen, "("),
            token!(TokenType::RightParen, ")"),
            token!(TokenType::LeftBrace, "{"),
            token!(TokenType::RightBrace, "}"),
            token!(TokenType::Comma, ","),
            token!(TokenType::Minus, "-"),
            token!(TokenType::Dot, "."),
            token!(TokenType::Semicolon, ";"),
            token!(TokenType::Plus, "+"),
            token!(TokenType::Star, "*"),
        ]
    );

    test!(
        single_chars_repeated,
        "(){}**--,-.;+*",
        vec![
            token!(TokenType::LeftParen, "("),
            token!(TokenType::RightParen, ")"),
            token!(TokenType::LeftBrace, "{"),
            token!(TokenType::RightBrace, "}"),
            token!(TokenType::Star, "*"),
            token!(TokenType::Star, "*"),
            token!(TokenType::Minus, "-"),
            token!(TokenType::Minus, "-"),
            token!(TokenType::Comma, ","),
            token!(TokenType::Minus, "-"),
            token!(TokenType::Dot, "."),
            token!(TokenType::Semicolon, ";"),
            token!(TokenType::Plus, "+"),
            token!(TokenType::Star, "*"),
        ]
    );

    test!(
        strings,
        "\"singleword\"",
        vec![token!(
            TokenType::String("singleword".into()),
            "\"singleword\""
        )]
    );

    test!(
        strings_and_chars_and_lines,
        "var x = \"singleword\"\n{}",
        vec![
            token!(TokenType::Var, "var", 0),
            token!(TokenType::Identifier("x".into()), "x", 0),
            token!(TokenType::Equal, "=", 0),
            token!(TokenType::String("singleword".into()), "\"singleword\"", 0),
            token!(TokenType::LeftBrace, "{", 1),
            token!(TokenType::RightBrace, "}", 1)
        ]
    );

    test!(
        strings_and_chars_and_lines_extended,
        "var x = \"singleword\";\nvar y = 2 + 3.;\nif (y >= 5.42 or y < 0.0000) quit();",
        vec![
            token!(TokenType::Var, "var", 0),
            token!(TokenType::Identifier("x".into()), "x", 0),
            token!(TokenType::Equal, "=", 0),
            token!(TokenType::String("singleword".into()), "\"singleword\"", 0),
            token!(TokenType::Semicolon, ";", 0),
            token!(TokenType::Var, "var", 1),
            token!(TokenType::Identifier("y".into()), "y", 1),
            token!(TokenType::Equal, "=", 1),
            token!(TokenType::Number(2f64), "2", 1),
            token!(TokenType::Plus, "+", 1),
            token!(TokenType::Number(3f64), "3.", 1),
            token!(TokenType::Semicolon, ";", 1),
            token!(TokenType::If, "if", 2),
            token!(TokenType::LeftParen, "(", 2),
            token!(TokenType::Identifier("y".into()), "y", 2),
            token!(TokenType::GreaterEqual, ">=", 2),
            token!(TokenType::Number(5.42f64), "5.42", 2),
            token!(TokenType::Or, "or", 2),
            token!(TokenType::Identifier("y".into()), "y", 2),
            token!(TokenType::Less, "<", 2),
            token!(TokenType::Number(0f64), "0.0000", 2),
            token!(TokenType::RightParen, ")", 2),
            token!(TokenType::Identifier("quit".into()), "quit", 2),
            token!(TokenType::LeftParen, "(", 2),
            token!(TokenType::RightParen, ")", 2),
            token!(TokenType::Semicolon, ";", 2),
        ]
    );
}
