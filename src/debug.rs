use crate::chunk::{Chunk, Instruction};

pub struct Disassembler<'code> {
    chunk: &'code Chunk,
}

impl<'code> Disassembler<'code> {
    pub fn new(chunk: &'code Chunk) -> Self {
        Self { chunk }
    }

    pub fn run(&self, name: &str) {
        println!("== {} ==", name);

        for (offset, inst) in self.chunk.code.iter().enumerate() {
            self.instruction(offset, *inst);
        }
    }

    fn instruction(&self, offset: usize, inst: Instruction) {
        print!("{:04} ", offset);

        let line = self.chunk.get_line(offset);

        if offset > 0 && line == self.chunk.get_line(offset - 1) {
            print!("   | ");
        } else {
            print!("{:4} ", line);
        }

        match inst {
            Instruction::Return => self.simple_instruction("OP_RETURN"),
            Instruction::Constant(idx) => self.constant_instruction("OP_CONSTANT", idx),
        }
    }

    fn simple_instruction(&self, msg: &'static str) {
        println!("{}", msg);
    }

    fn constant_instruction(&self, msg: &'static str, idx: u16) {
        let value = self.chunk.constants[idx as usize];
        println!("{:<16} {:4} ({:?})", msg, idx, value);
    }
}
