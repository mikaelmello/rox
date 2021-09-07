use crate::{
    chunk::{Chunk, Instruction, Value},
    debug::Disassembler,
    error::{CompilationError, RoxError, RoxErrorKind, RoxResult},
    heap::Heap,
    scanner::{scanner::TokenIter, token::TokenErrorKind, Scanner, Token, TokenKind},
};

#[derive(Copy, Clone, PartialOrd, PartialEq)]
enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

impl Precedence {
    fn next(&self) -> Precedence {
        match self {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::None,
        }
    }
}

type ParseFn<'sourcecode> = fn(&mut Parser<'sourcecode>) -> RoxResult<()>;

#[derive(Copy, Clone)]
struct ParseRule<'sourcecode> {
    prefix: Option<ParseFn<'sourcecode>>,
    infix: Option<ParseFn<'sourcecode>>,
    precedence: Precedence,
}

impl<'sourcecode>
    From<(
        Option<ParseFn<'sourcecode>>,
        Option<ParseFn<'sourcecode>>,
        Precedence,
    )> for ParseRule<'sourcecode>
{
    fn from(
        (prefix, infix, precedence): (
            Option<ParseFn<'sourcecode>>,
            Option<ParseFn<'sourcecode>>,
            Precedence,
        ),
    ) -> Self {
        Self {
            prefix,
            infix,
            precedence,
        }
    }
}

struct Parser<'sourcecode> {
    scanner: TokenIter<'sourcecode>,
    current: Token<'sourcecode>,
    previous: Token<'sourcecode>,
    heap: &'sourcecode mut Heap,
    chunks: Vec<Chunk>,
    errors: Vec<RoxError>,
}

impl<'sourcecode> Parser<'sourcecode> {
    pub fn new(code: &'sourcecode str, heap: &'sourcecode mut Heap) -> Self {
        Self {
            scanner: Scanner::new(code).into_iter(),
            previous: Token::synthetic(""),
            current: Token::synthetic(""),
            chunks: Vec::new(),
            errors: Vec::new(),
            heap,
        }
    }

    pub fn compile(mut self) -> Result<Chunk, Vec<RoxError>> {
        self.chunks.push(Chunk::new());

        match self.advance().and_then(|_| self.expression()) {
            Ok(()) => {}
            Err(err) => self.errors.push(err),
        }

        // if !matches!(self.current.kind(), TokenKind::Eof) {
        //     let error = self.error_at_current(CompilationError::UnexpectedToken(
        //         self.current.lexeme().to_string(),
        //     ));
        //     self.errors.push(error);
        // }

        self.end_compiler();

        if self.errors.is_empty() {
            Ok(self.chunks.pop().unwrap())
        } else {
            Err(self.errors)
        }
    }

    fn expression(&mut self) -> RoxResult<()> {
        self.parse_precedence(Precedence::Assignment)
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> RoxResult<()> {
        self.advance()?;

        let rule = self.get_rule(self.previous.kind());
        let prefix_rule = rule.prefix;

        let prefix_rule = match prefix_rule {
            Some(rule) => rule,
            None => {
                return Err(self.error(CompilationError::MissingExpression));
            }
        };

        prefix_rule(self)?;

        while precedence <= self.get_rule(self.current.kind()).precedence {
            self.advance()?;
            println!("{:?} at main loop", self.previous.kind());
            println!("{:?} at main loop", self.current.kind());
            let infix_rule = self
                .get_rule(self.previous.kind())
                .infix
                .expect("Expect infix rule");
            infix_rule(self)?;
        }

        Ok(())
    }

    fn grouping(&mut self) -> RoxResult<()> {
        self.expression()?;
        self.consume(
            TokenKind::RightParen,
            CompilationError::MissingClosingParenthesis,
        )
    }

    fn binary(&mut self) -> RoxResult<()> {
        let operator = self.previous.kind();
        let rule = self.get_rule(operator);
        self.parse_precedence(rule.precedence.next())?;

        match operator {
            TokenKind::Plus => self.emit(Instruction::Add),
            TokenKind::Minus => self.emit(Instruction::Subtract),
            TokenKind::Star => self.emit(Instruction::Multiply),
            TokenKind::Slash => self.emit(Instruction::Divide),
            TokenKind::BangEqual => self.emit_many(&[Instruction::Equal, Instruction::Not]),
            TokenKind::EqualEqual => self.emit(Instruction::Equal),
            TokenKind::Greater => self.emit(Instruction::Greater),
            TokenKind::GreaterEqual => self.emit_many(&[Instruction::Less, Instruction::Not]),
            TokenKind::Less => self.emit(Instruction::Less),
            TokenKind::LessEqual => self.emit_many(&[Instruction::Greater, Instruction::Not]),
            _ => panic!("Invalid binary operator"),
        }

        Ok(())
    }

    fn unary(&mut self) -> RoxResult<()> {
        let kind = self.previous.kind();

        self.parse_precedence(Precedence::Unary)?;

        match kind {
            TokenKind::Minus => self.emit(Instruction::Negate),
            TokenKind::Bang => self.emit(Instruction::Not),
            _ => panic!("Invalid unary operator"),
        }

        Ok(())
    }

    fn literal(&mut self) -> RoxResult<()> {
        match self.previous.kind() {
            TokenKind::False => self.emit(Instruction::False),
            TokenKind::True => self.emit(Instruction::True),
            TokenKind::Nil => self.emit(Instruction::Nil),
            _ => panic!("Invalid literal token"),
        }

        Ok(())
    }

    fn number(&mut self) -> RoxResult<()> {
        assert!(matches!(self.previous.kind(), TokenKind::Number));

        match self.previous.lexeme().parse::<f64>() {
            Ok(value) => {
                self.emit_constant(Value::Number(value))?;
                Ok(())
            }
            Err(_) => Err(self.error(CompilationError::InvalidNumberLiteral(
                self.previous.lexeme().into(),
            ))),
        }
    }

    fn string(&mut self) -> RoxResult<()> {
        assert!(matches!(self.previous.kind(), TokenKind::String));

        let lexeme = self.previous.lexeme();
        let value = &lexeme[1..(lexeme.len() - 1)];

        let reference = self.heap.alloc_string(String::from(value));
        self.emit_constant(Value::String(reference))?;

        Ok(())
    }

    fn get_rule(&mut self, kind: TokenKind) -> ParseRule<'sourcecode> {
        let rule: (
            Option<ParseFn<'sourcecode>>,
            Option<ParseFn<'sourcecode>>,
            Precedence,
        ) = match kind {
            TokenKind::LeftParen => (Some(Self::grouping), None, Precedence::None),
            TokenKind::RightParen => (None, None, Precedence::None),
            TokenKind::LeftBrace => (None, None, Precedence::None),
            TokenKind::RightBrace => (None, None, Precedence::None),
            TokenKind::Comma => (None, None, Precedence::None),
            TokenKind::Dot => (None, None, Precedence::None),
            TokenKind::Minus => (Some(Self::unary), Some(Self::binary), Precedence::Term),
            TokenKind::Plus => (None, Some(Self::binary), Precedence::Term),
            TokenKind::Semicolon => (None, None, Precedence::None),
            TokenKind::Slash => (None, Some(Self::binary), Precedence::Factor),
            TokenKind::Star => (None, Some(Self::binary), Precedence::Factor),
            TokenKind::Bang => (Some(Self::unary), None, Precedence::None),
            TokenKind::BangEqual => (None, Some(Self::binary), Precedence::Equality),
            TokenKind::Equal => (None, None, Precedence::None),
            TokenKind::EqualEqual => (None, Some(Self::binary), Precedence::Equality),
            TokenKind::Greater => (None, Some(Self::binary), Precedence::Comparison),
            TokenKind::GreaterEqual => (None, Some(Self::binary), Precedence::Comparison),
            TokenKind::Less => (None, Some(Self::binary), Precedence::Comparison),
            TokenKind::LessEqual => (None, Some(Self::binary), Precedence::Comparison),
            TokenKind::Identifier => (None, None, Precedence::None),
            TokenKind::String => (Some(Self::string), None, Precedence::None),
            TokenKind::Number => (Some(Self::number), None, Precedence::None),
            TokenKind::And => (None, None, Precedence::None),
            TokenKind::Class => (None, None, Precedence::None),
            TokenKind::Else => (None, None, Precedence::None),
            TokenKind::False => (Some(Self::literal), None, Precedence::None),
            TokenKind::Fun => (None, None, Precedence::None),
            TokenKind::For => (None, None, Precedence::None),
            TokenKind::If => (None, None, Precedence::None),
            TokenKind::Nil => (Some(Self::literal), None, Precedence::None),
            TokenKind::Or => (None, None, Precedence::None),
            TokenKind::Print => (None, None, Precedence::None),
            TokenKind::Return => (None, None, Precedence::None),
            TokenKind::Super => (None, None, Precedence::None),
            TokenKind::This => (None, None, Precedence::None),
            TokenKind::True => (Some(Self::literal), None, Precedence::None),
            TokenKind::Var => (None, None, Precedence::None),
            TokenKind::While => (None, None, Precedence::None),
            TokenKind::Error(_) => (None, None, Precedence::None),
            TokenKind::Eof => (None, None, Precedence::None),
        };

        ParseRule::from(rule)
    }

    fn emit(&mut self, instruction: Instruction) {
        let line = self.previous.location().line();
        self.current_chunk().write(instruction, line);
    }

    fn emit_many(&mut self, instructions: &[Instruction]) {
        for inst in instructions {
            self.emit(*inst);
        }
    }

    fn emit_constant(&mut self, value: Value) -> RoxResult<()> {
        let index = self.make_constant(value)?;
        self.emit(Instruction::Constant(index));

        Ok(())
    }

    fn emit_return(&mut self) {
        self.emit(Instruction::Return)
    }

    fn end_compiler(&mut self) {
        if !self.errors.is_empty() {
            #[cfg(feature = "debug_trace_execution")]
            {
                let dis = Disassembler::new(self.current_chunk(), None);
                dis.run("Finished compiling");
            }
        }
        self.emit_return();
    }

    fn make_constant(&mut self, value: Value) -> RoxResult<u16> {
        match self.current_chunk().add_constant(value) {
            Ok(index) => Ok(index),
            Err(_) => Err(self.error(CompilationError::TooManyConstants(u16::MAX as u64))),
        }
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        self.chunks.last_mut().unwrap()
    }

    fn consume(&mut self, kind: TokenKind, error: CompilationError) -> RoxResult<()> {
        if self.current.kind() == kind {
            self.advance()?;
            return Ok(());
        }

        Err(self.error_at_current(error))
    }

    fn advance(&mut self) -> RoxResult<()> {
        self.previous = self.current;

        if let Some(token) = self.scanner.next() {
            self.current = token;

            match token.kind() {
                TokenKind::Error(TokenErrorKind::InvalidLexeme) => {
                    return Err(self.error(CompilationError::InvalidLexeme(token.lexeme().into())))
                }
                TokenKind::Error(TokenErrorKind::SyntheticToken) => {
                    panic!("Unexpected synthetic token, this is a bug in the compiler");
                }
                TokenKind::Error(TokenErrorKind::UnterminatedString) => {
                    return Err(self.error(CompilationError::UnterminatedString))
                }
                _ => {}
            }

            println!("{:?} {:?}", self.previous.kind(), self.current.kind());
        }

        Ok(())
    }

    fn error_at_current(&mut self, kind: CompilationError) -> RoxError {
        self.error_at(self.current, kind)
    }

    fn error(&mut self, kind: CompilationError) -> RoxError {
        self.error_at(self.previous, kind)
    }

    fn error_at(&mut self, token: Token, kind: CompilationError) -> RoxError {
        RoxError::new(
            RoxErrorKind::CompilationError(kind),
            token.location().line(),
        )
    }
}

pub fn compile(code: &str, heap: &mut Heap) -> Result<Chunk, Vec<RoxError>> {
    Parser::new(code, heap).compile()
}
