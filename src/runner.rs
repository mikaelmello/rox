use std::fs;

use crate::{chunk::Chunk, vm::Vm};

pub fn eval_file(path: &str) {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let _ = eval(&contents);
}

pub fn eval(expr: &str) {
    let mut vm = Vm::new(Chunk::new());

    if let Err(errors) = vm.interpret(expr) {
        for err in errors {
            eprintln!("[line {}] Error: {}", err.line, err.src);
        }
    }
}
