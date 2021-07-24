use crate::chunk::{Chunk, Instruction, Value};

pub struct Disassembler<'vm> {
    chunk: &'vm Chunk,
    stack: Option<&'vm Vec<Value>>,
}

impl<'vm> Disassembler<'vm> {
    pub fn new(chunk: &'vm Chunk, stack: Option<&'vm Vec<Value>>) -> Self {
        Self { chunk, stack }
    }

    pub fn run(&self, name: &str) {
        println!("== {} ==", name);

        for (offset, inst) in self.chunk.code.iter().enumerate() {
            self.instruction(offset, *inst);
        }
    }

    pub fn instruction(&self, offset: usize, inst: Instruction) {
        self.stack();

        print!("{:04} ", offset);

        let line = self.chunk.get_line(offset);

        if offset > 0 && line == self.chunk.get_line(offset - 1) {
            print!("   | ");
        } else {
            print!("{:4} ", line);
        }

        match inst {
            Instruction::Return => self.simple_instruction("OP_RETURN"),
            Instruction::Negate => self.simple_instruction("OP_NEGATE"),
            Instruction::Constant(idx) => self.constant_instruction("OP_CONSTANT", idx),
            Instruction::Add => self.simple_instruction("OP_ADD"),
            Instruction::Subtract => self.simple_instruction("OP_SUB"),
            Instruction::Multiply => self.simple_instruction("OP_MUL"),
            Instruction::Divide => self.simple_instruction("OP_DIV"),
        }
    }

    fn simple_instruction(&self, msg: &'static str) {
        println!("{}", msg);
    }

    fn constant_instruction(&self, msg: &'static str, idx: u16) {
        let value = self.chunk.constants[idx as usize];
        println!("{:<16} {:4} ({:?})", msg, idx, value);
    }

    fn stack(&self) {
        if let Some(stack) = self.stack {
            print!(" S: ");
            for &value in stack.iter() {
                print!("[{:?}]", value);
            }
            println!();
        }
    }
}
