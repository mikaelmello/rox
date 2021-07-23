use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io::{self, Write};

use crate::runner;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn repl() -> io::Result<()> {
    let mut stdout = io::stdout();

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();

    writeln!(stdout, "rox {}", VERSION)?;

    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }

                rl.add_history_entry(line.as_str());

                let results = runner::eval(&line);

                for result in results {
                    if !result.is_empty() {
                        write!(stdout, "{}\n", result)?;
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
