use std::io::{self, BufRead, Write};

use crate::runner;

pub fn repl() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        buffer.clear();

        write!(stdout, "> ")?;
        stdout.flush()?;

        let read = stdin.lock().read_line(&mut buffer)?;

        if read == 0 {
            write!(stdout, "\n")?;
            break;
        }

        let result = runner::eval(&buffer);

        write!(stdout, "{}\n", result)?;
    }
    Ok(())
}
