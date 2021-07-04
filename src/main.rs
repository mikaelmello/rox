use clap::Clap;
use opts::Opts;

mod opts;

fn main() {
    let opts: Opts = Opts::parse();
}
