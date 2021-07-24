use crate::{
    chunk::{Chunk, Instruction, Value},
    debug::Disassembler,
    scanner::{scanner::TokenIter, Scanner, Token, TokenKind},
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

type ParseFn<'sourcecode> = fn(&mut Parser<'sourcecode>) -> ();

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
    chunks: Vec<Chunk>,
    had_error: bool,
    panic_mode: bool,
}

impl<'sourcecode> Parser<'sourcecode> {
    pub fn new(code: &'sourcecode str) -> Self {
        Self {
            scanner: Scanner::new(code).into_iter(),
            previous: Token::synthetic(""),
            current: Token::synthetic(""),
            had_error: false,
            panic_mode: false,
            chunks: Vec::new(),
        }
    }

    pub fn compile(mut self) -> Result<Chunk, ()> {
        self.chunks.push(Chunk::new());

        self.advance();
        self.expression();
        self.end_compiler();

        if self.had_error {
            Err(())
        } else {
            Ok(self.chunks.pop().unwrap())
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        let rule = self.get_rule(self.previous.kind());
        let prefix_rule = rule.prefix;

        let prefix_rule = match prefix_rule {
            Some(rule) => rule,
            None => {
                self.error("Expect expression.");
                return;
            }
        };

        prefix_rule(self);

        while precedence <= self.get_rule(self.current.kind()).precedence {
            self.advance();
            let infix_rule = self
                .get_rule(self.previous.kind())
                .infix
                .expect("Expect infix rule");
            infix_rule(self);
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenKind::RightParen, "Expect ')' after expression")
    }

    fn binary(&mut self) {
        let operator = self.previous.kind();
        let rule = self.get_rule(operator);
        self.parse_precedence(rule.precedence.next());

        match operator {
            TokenKind::Plus => self.emit(Instruction::Add),
            TokenKind::Minus => self.emit(Instruction::Subtract),
            TokenKind::Star => self.emit(Instruction::Multiply),
            TokenKind::Slash => self.emit(Instruction::Divide),
            _ => panic!("Invalid binary operator"),
        };
    }

    fn unary(&mut self) {
        let kind = self.previous.kind();

        self.parse_precedence(Precedence::Unary);

        match kind {
            TokenKind::Minus => self.emit(Instruction::Negate),
            _ => panic!("Invalid unary operator"),
        }
    }

    fn number(&mut self) {
        let value = match self.previous.lexeme().parse::<f64>() {
            Ok(value) => value,
            Err(_) => {
                self.error("Invalid number lexeme");
                return;
            }
        };
        self.emit_constant(Value::Number(value));
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
            TokenKind::Bang => (None, None, Precedence::None),
            TokenKind::BangEqual => (None, None, Precedence::None),
            TokenKind::Equal => (None, None, Precedence::None),
            TokenKind::EqualEqual => (None, None, Precedence::None),
            TokenKind::Greater => (None, None, Precedence::None),
            TokenKind::GreaterEqual => (None, None, Precedence::None),
            TokenKind::Less => (None, None, Precedence::None),
            TokenKind::LessEqual => (None, None, Precedence::None),
            TokenKind::Identifier => (None, None, Precedence::None),
            TokenKind::String => (None, None, Precedence::None),
            TokenKind::Number => (Some(Self::number), None, Precedence::None),
            TokenKind::And => (None, None, Precedence::None),
            TokenKind::Class => (None, None, Precedence::None),
            TokenKind::Else => (None, None, Precedence::None),
            TokenKind::False => (None, None, Precedence::None),
            TokenKind::Fun => (None, None, Precedence::None),
            TokenKind::For => (None, None, Precedence::None),
            TokenKind::If => (None, None, Precedence::None),
            TokenKind::Nil => (None, None, Precedence::None),
            TokenKind::Or => (None, None, Precedence::None),
            TokenKind::Print => (None, None, Precedence::None),
            TokenKind::Return => (None, None, Precedence::None),
            TokenKind::Super => (None, None, Precedence::None),
            TokenKind::This => (None, None, Precedence::None),
            TokenKind::True => (None, None, Precedence::None),
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

    fn emit_constant(&mut self, value: Value) {
        let index = self.make_constant(value);
        self.emit(Instruction::Constant(index));
    }

    fn emit_return(&mut self) {
        self.emit(Instruction::Return)
    }

    fn end_compiler(&mut self) {
        if !self.had_error {
            #[cfg(feature = "debug_trace_execution")]
            {
                let dis = Disassembler::new(self.current_chunk(), None);
                dis.run("Finished compiling");
            }
        }
        self.emit_return();
    }

    fn make_constant(&mut self, value: Value) -> u16 {
        match self.current_chunk().add_constant(value) {
            Ok(index) => index,
            Err(_) => {
                self.error("Too many constants in one chunk.");
                0
            }
        }
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        self.chunks.last_mut().unwrap()
    }

    fn consume(&mut self, kind: TokenKind, msg: &str) {
        println!("{:?} {:?}", self.current.kind(), kind);
        if self.current.kind() == kind {
            self.advance();
            return;
        }

        self.error_at_current(msg);
    }

    fn advance(&mut self) {
        self.previous = self.current;

        while let Some(token) = self.scanner.next() {
            self.current = token;
            if token.is_error() {
                todo!()
            } else {
                break;
            }
        }
    }

    fn error_at_current(&mut self, msg: &str) {
        self.error_at(self.current, msg)
    }

    fn error(&mut self, msg: &str) {
        self.error_at(self.previous, msg)
    }

    fn error_at(&mut self, token: Token, msg: &str) {
        if self.panic_mode {
            return;
        }

        self.panic_mode = true;
        self.had_error = true;

        eprint!("[{}] Error", token.location());

        match token.kind() {
            TokenKind::Error(_) => (),
            _ => eprint!(" at '{}'", token.lexeme()),
        };
        eprintln!(": {}", msg);
    }
}

pub fn compile(code: &str) -> Result<Chunk, ()> {
    Parser::new(code).compile()
}
