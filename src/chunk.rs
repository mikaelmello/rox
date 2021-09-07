use std::convert::TryFrom;

use crate::heap::Ref;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Instruction {
    Return,
    Constant(u16),
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    False,
    Nil,
    True,
    Not,
    Equal,
    Greater,
    Less,
}

#[derive(Copy, Clone, Debug)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
    String(Ref<String>),
}

impl Value {
    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Number(_) => false,
            Value::Bool(val) => !val,
            Value::Nil => true,
            Value::String(_) => false,
        }
    }
}

pub struct LineStart {
    offset: usize,
    line: usize,
}

impl LineStart {
    pub fn new(offset: usize, line: usize) -> Self {
        Self { offset, line }
    }
}

pub struct Chunk {
    pub code: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub lines: Vec<LineStart>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, instruction: Instruction, line: usize) -> usize {
        let index = self.code.len();

        self.code.push(instruction);

        match self.lines.last() {
            Some(cur_line) if cur_line.line == line => {}
            _ => self.lines.push(LineStart::new(index, line)),
        };

        index
    }

    pub fn add_constant(&mut self, value: Value) -> Result<u16, ()> {
        let index = self.constants.len();

        match u16::try_from(index) {
            Ok(index) => {
                self.constants.push(value);
                Ok(index)
            }
            Err(_) => Err(()),
        }
    }

    pub fn get_line(&self, instruction_idx: usize) -> usize {
        assert!(
            instruction_idx < self.code.len(),
            "Do not try to get line of instruction not added to chunk"
        );
        assert!(
            !self.lines.is_empty(),
            "Do not try to get line index when none were added"
        );

        let mut left = 0;
        let mut right = self.lines.len() - 1;

        let mut line = self.lines.last().expect("Lines is empty").line;

        while left <= right {
            let mid = (left + right) / 2;

            match self.lines.get(mid) {
                Some(mid_line) => {
                    if instruction_idx >= mid_line.offset {
                        line = mid_line.line;

                        if mid == 0 {
                            break;
                        }
                        right = mid - 1;
                    } else {
                        left = mid + 1;
                    }
                }
                None => panic!("Invalid mid index when looking for line"),
            }
        }

        line
    }
}

#[cfg(test)]
mod test {
    use std::mem::size_of;

    use crate::chunk::{Instruction, Value};

    #[test]
    fn instruction_is_at_most_32_bits() {
        // An instruction should be at most 32 bits; anything bigger and we've mis-defined some
        // variant
        assert!(size_of::<Instruction>() <= 4);
    }

    #[test]
    fn value_is_at_most_32_bits() {
        // An instruction should be at most 128 bits; anything bigger and we've mis-defined some
        // variant
        assert!(size_of::<Value>() <= 16);
    }
}
