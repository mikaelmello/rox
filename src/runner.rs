use std::{fs, io::Cursor};

use crate::lexer::scanner::Scanner;

pub fn eval_file(path: &str) {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let _ = eval(&contents);
}

pub fn eval(expr: &str) -> String {
    let scanner = Scanner::from(Cursor::new(expr)).unwrap();

    let tokens = scanner.scan_tokens();

    match tokens {
        Ok(t) => format!("{:?}", t),
        Err(e) => format!("{}", e),
    }
}
