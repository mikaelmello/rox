use std::{
    fs::File,
    io::{BufReader, Bytes, Read, Seek},
};

use unicode_reader::{CodePoints, Graphemes};

use crate::error::RoxError;

use super::token::{Token, TokenType};

enum CodeSource {
    File(File),
    String(String),
}

struct Scanner<T: Read + Seek> {
    graphemes: Graphemes<CodePoints<Bytes<BufReader<T>>>>,
    line: usize,
}

impl<T: Read + Seek> Scanner<T> {
    pub fn from(file: T) -> Result<Self, RoxError> {
        let reader = BufReader::new(file);
        let graphemes = Graphemes::from(reader);

        Ok(Self { graphemes, line: 0 })
    }

    pub fn scan_tokens(self) -> Result<Vec<Token>, RoxError> {
        Ok(vec![])
    }

    fn next_token(&mut self) -> Result<Option<Token>, RoxError> {
        let grapheme = match self.advance()? {
            Some(s) => s,
            None => return Ok(None),
        };

        macro_rules! token {
            ($type:expr) => {
                Ok(Some(Token::new($type, grapheme, self.line)))
            };
        }

        if grapheme.len() == 1 {
            let first_char = match grapheme.chars().next() {
                None => return Ok(None),
                Some(c) => c,
            };

            match first_char {
                '(' => return token!(TokenType::LeftParen),
                ')' => return token!(TokenType::RightParen),
                '{' => return token!(TokenType::LeftBrace),
                '}' => return token!(TokenType::RightBrace),
                ',' => return token!(TokenType::Comma),
                '.' => return token!(TokenType::Dot),
                '-' => return token!(TokenType::Minus),
                '+' => return token!(TokenType::Plus),
                ';' => return token!(TokenType::Semicolon),
                '*' => return token!(TokenType::Star),
                _ => {}
            };
        }

        token!(TokenType::Star)
    }

    fn advance(&mut self) -> Result<Option<String>, RoxError> {
        Ok(self.graphemes.next().transpose()?)
    }
}
