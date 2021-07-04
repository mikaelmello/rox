use clap::Clap;
use opts::Opts;

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
