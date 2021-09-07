use super::{token::TokenErrorKind, Token, TokenKind};
use crate::location::Location;

pub struct Scanner<'sourcecode> {
    code: &'sourcecode str,
    code_bytes: &'sourcecode [u8],
    start: Location,
    current: Location,
}

impl<'sourcecode> Scanner<'sourcecode> {
    pub fn new(code: &'sourcecode str) -> Scanner {
        Scanner {
            code,
            code_bytes: code.as_bytes(),
            start: Location::default(),
            current: Location::default(),
        }
    }

    #[allow(unused)]
    pub fn next_token(&mut self) -> Token<'sourcecode> {
        self.skip_non_tokens();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenKind::Eof);
        }

        let mut invalid_lexeme = false;

        macro_rules! check_invalid_lexeme {
            ($expr:expr) => {{
                if invalid_lexeme {
                    return self.error_token(TokenErrorKind::InvalidLexeme);
                }
                return $expr;
            }};
        }

        macro_rules! token {
            ($kind:expr) => {
                token!($kind, 1);
            };
            ($kind:expr,$qty:expr) => {{
                check_invalid_lexeme! {{
                    for _ in 0..$qty {
                        self.advance();
                    }
                    self.make_token($kind)
                }}
            }};
        }

        loop {
            match self.peek() {
                b'(' => token!(TokenKind::LeftParen),
                b')' => token!(TokenKind::RightParen),
                b'{' => token!(TokenKind::LeftBrace),
                b'}' => token!(TokenKind::RightBrace),
                b';' => token!(TokenKind::Semicolon),
                b',' => token!(TokenKind::Comma),
                b'.' => token!(TokenKind::Dot),
                b'-' => token!(TokenKind::Minus),
                b'+' => token!(TokenKind::Plus),
                b'/' => token!(TokenKind::Slash),
                b'*' => token!(TokenKind::Star),

                b'!' if self.peek_next() == b'=' => token!(TokenKind::BangEqual, 2),
                b'!' => token!(TokenKind::Bang),

                b'=' if self.peek_next() == b'=' => token!(TokenKind::EqualEqual, 2),
                b'=' => token!(TokenKind::Equal),

                b'<' if self.peek_next() == b'=' => token!(TokenKind::LessEqual, 2),
                b'<' => token!(TokenKind::Less),

                b'>' if self.peek_next() == b'=' => token!(TokenKind::GreaterEqual, 2),
                b'>' => token!(TokenKind::Greater),

                b'"' => check_invalid_lexeme!(self.string()),
                c if is_digit(c) => check_invalid_lexeme!(self.number()),
                c if is_alpha(c) => check_invalid_lexeme!(self.identifier()),
                _ => {
                    self.advance();
                    invalid_lexeme = true;
                }
            };
        }
    }

    fn skip_non_tokens(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                b' ' | b'\r' | b'\t' | b'\n' => {
                    self.advance();
                }
                b'/' if self.peek_next() == b'/' => {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                _ => return,
            }
        }
    }

    fn string(&mut self) -> Token<'sourcecode> {
        self.advance();

        while self.peek() != b'"' && !self.is_at_end() {
            self.advance();
        }

        let x = 3 u64;

        if self.is_at_end() {
            self.error_token(TokenErrorKind::UnterminatedString)
        } else {
            self.advance();
            self.make_token(TokenKind::String)
        }
    }

    fn number(&mut self) -> Token<'sourcecode> {
        self.advance();

        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == b'.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }

        self.make_token(TokenKind::Number)
    }

    fn identifier(&mut self) -> Token<'sourcecode> {
        self.advance();
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }

        if let Some(reserved) = reserved_token(self.cur_lexeme()) {
            self.make_token(reserved)
        } else {
            self.make_token(TokenKind::Identifier)
        }
    }

    fn error_token(&self, kind: TokenErrorKind) -> Token<'sourcecode> {
        Token::new(
            TokenKind::Error(kind),
            self.cur_lexeme(),
            self.start,
            self.current,
        )
    }

    fn advance(&mut self) -> u8 {
        let c = self.peek();
        self.current.advance(c);
        c
    }

    fn peek(&mut self) -> u8 {
        self.peek_far(0)
    }

    fn peek_next(&mut self) -> u8 {
        self.peek_far(1)
    }

    fn peek_far(&mut self, dist: usize) -> u8 {
        let idx = self.current.offset().saturating_add(dist);
        if idx >= self.code_bytes.len() {
            0
        } else {
            self.code_bytes[idx]
        }
    }

    fn cur_lexeme(&self) -> &'sourcecode str {
        &self.code[self.start.offset()..self.current.offset()]
    }

    fn is_at_end(&self) -> bool {
        self.current.offset() == self.code.len()
    }

    fn make_token(&self, kind: TokenKind) -> Token<'sourcecode> {
        Token::new(kind, self.cur_lexeme(), self.start, self.current)
    }

    pub fn into_iter(self) -> TokenIter<'sourcecode> {
        TokenIter { scanner: self }
    }
}
pub struct TokenIter<'sourcecode> {
    scanner: Scanner<'sourcecode>,
}

impl<'sourcecode> Iterator for TokenIter<'sourcecode> {
    type Item = Token<'sourcecode>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.scanner.next_token();

        if token.kind() == TokenKind::Eof {
            None
        } else {
            Some(token)
        }
    }
}

#[inline]
fn is_digit(c: u8) -> bool {
    c.is_ascii_digit()
}

#[inline]
fn is_alpha(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_'
}

#[inline]
pub fn reserved_token(lexeme: &str) -> Option<TokenKind> {
    match lexeme {
        "and" => Some(TokenKind::And),
        "class" => Some(TokenKind::Class),
        "else" => Some(TokenKind::Else),
        "false" => Some(TokenKind::False),
        "for" => Some(TokenKind::For),
        "fun" => Some(TokenKind::Fun),
        "if" => Some(TokenKind::If),
        "nil" => Some(TokenKind::Nil),
        "or" => Some(TokenKind::Or),
        "print" => Some(TokenKind::Print),
        "return" => Some(TokenKind::Return),
        "super" => Some(TokenKind::Super),
        "this" => Some(TokenKind::This),
        "true" => Some(TokenKind::True),
        "var" => Some(TokenKind::Var),
        "while" => Some(TokenKind::While),
        _ => None,
    }
}
