use clap::Clap;
use opts::Opts;

mod opts;

fn main() {
    let opts: Opts = Opts::parse();

    match opts.script {
        Some(path) => {}
        None => {}
    }
}
