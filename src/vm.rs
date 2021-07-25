use crate::{
    chunk::{Chunk, Instruction, Value},
    compiler::compile,
    debug::Disassembler,
    error::{RoxError, RoxErrorKind, RoxResult, RuntimeError},
};
use core::panic;

pub struct Vm {
    ip: usize,
    chunk: Chunk,
    stack: Vec<Value>,
}

impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, code: &str) -> Result<(), Vec<RoxError>> {
        self.chunk = compile(code)?;
        self.ip = 0;

        self.run().map_err(|err| vec![err])
    }

    fn run(&mut self) -> RoxResult<()> {
        loop {
            let inst = match self.chunk.code.get(self.ip) {
                Some(inst) => inst,
                None => panic!("Reached out-of-bounds of program"),
            };

            #[cfg(feature = "debug_trace_execution")]
            {
                let dis = Disassembler::new(&self.chunk, Some(&self.stack));
                dis.instruction(self.ip, *inst);
            }

            self.ip = self.ip.saturating_add(1);

            macro_rules! binary_op {
                ($oper:tt) => {{
                    let b = match self.stack.pop() {
                        Some(Value::Number(val)) => val,
                        Some(_) => Err(self.runtime_error(RuntimeError::InvalidOperand))?,
                        None => Err(self.runtime_error(RuntimeError::MissingOperand))?,
                    };
                    let a = match self.stack.pop() {
                        Some(Value::Number(val)) => val,
                        Some(_) => Err(self.runtime_error(RuntimeError::InvalidOperand))?,
                        None => Err(self.runtime_error(RuntimeError::MissingOperand))?,
                    };
                    let res = a $oper b;
                    self.stack.push(Value::Number(res));
                }};
            }

            match inst {
                Instruction::Return => {
                    if let Some(val) = self.stack.pop() {
                        println!("{:?}", val);
                    }

                    return Ok(());
                }
                Instruction::Constant(val) => {
                    let val = match self.chunk.constants.get(*val as usize) {
                        Some(val) => val,
                        None => Err(self.runtime_error(RuntimeError::InvalidConstantAddress))?,
                    };

                    self.stack.push(*val);
                }
                Instruction::Negate => match self.stack.last_mut() {
                    Some(Value::Number(val)) => {
                        *val *= -1.0;
                    }
                    Some(_) => Err(self.runtime_error(RuntimeError::InvalidOperand))?,
                    None => Err(self.runtime_error(RuntimeError::MissingOperand))?,
                },
                Instruction::Add => binary_op!(+),
                Instruction::Subtract => binary_op!(-),
                Instruction::Multiply => binary_op!(*),
                Instruction::Divide => binary_op!(/),
            }
        }
    }

    fn runtime_error(&mut self, kind: RuntimeError) -> RoxError {
        RoxError::new(
            RoxErrorKind::RuntimeError(kind),
            self.chunk.get_line(self.ip),
        )
    }
}
