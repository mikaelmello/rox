use clap::Clap;
use opts::Opts;

mod ast;
mod error;
mod lexer;
mod opts;
mod parser;
mod repl;
mod runner;

fn main() {
    let opts: Opts = Opts::parse();

    match opts.script {
        Some(path) => runner::eval_file(&path),
        None => repl::repl().unwrap(),
    }
}
