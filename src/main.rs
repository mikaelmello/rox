use chunk::{Chunk, Instruction, Value};
use clap::Clap;
use debug::Disassembler;
use opts::Opts;

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
    code.write(Instruction::Return, 123);

    let disassembler = Disassembler::new(&code);
    disassembler.run("DEBUG");

    return;

    // let opts: Opts = Opts::parse();

    // match opts.script {
    //     Some(path) => runner::eval_file(&path),
    //     None => repl::repl().unwrap(),
    // }
}
