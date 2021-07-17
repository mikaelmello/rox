use ast::Expr;
use clap::Clap;
use lexer::{
    location::Location,
    token::{Token, TokenType},
};
use opts::Opts;

mod ast;
mod error;
mod lexer;
mod opts;
mod repl;
mod runner;

fn main() {
    let opts: Opts = Opts::parse();

    match opts.script {
        Some(path) => runner::eval_file(&path),
        None => repl::repl().unwrap(),
    }
}
