use std::fs;

use crate::{chunk::Chunk, scanner::Scanner, vm::Vm};

pub fn eval_file(path: &str) {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let _ = eval(&contents);
}

pub fn eval(expr: &str) -> String {
    let mut vm = Vm::new(Chunk::new());

    match vm.interpret(expr) {
        Ok(_) => "",
        Err(crate::error::RoxError::CompileError) => "Compilation error",
        Err(crate::error::RoxError::RuntimeError) => "Runtime error",
    }
    .into()
}
