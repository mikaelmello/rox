use clap::Clap;
use opts::Opts;

mod location;
mod opts;
mod repl;
mod runner;
mod scanner;

fn main() {
    let opts: Opts = Opts::parse();

    match opts.script {
        Some(path) => runner::eval_file(&path),
        None => repl::repl().unwrap(),
    }
}
