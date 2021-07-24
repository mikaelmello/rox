use chunk::{Chunk, Instruction, Value};
use debug::Disassembler;
use vm::Vm;

mod chunk;
mod debug;
mod error;
mod location;
mod opts;
mod repl;
mod runner;
mod scanner;
mod vm;

fn main() {
    let mut code = Chunk::new();
    let constant = code.add_constant(Value::Number(123.34)).unwrap();

    code.write(Instruction::Constant(constant), 123);
    code.write(Instruction::Negate, 123);
    code.write(Instruction::Constant(constant), 123);
    code.write(Instruction::Negate, 123);
    code.write(Instruction::Subtract, 123);
    code.write(Instruction::Return, 123);

    let mut vm = Vm::new(&code);
    vm.interpret().unwrap();

    return;

    // let opts: Opts = Opts::parse();

    // match opts.script {
    //     Some(path) => runner::eval_file(&path),
    //     None => repl::repl().unwrap(),
    // }
}
