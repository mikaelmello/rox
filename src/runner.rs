use std::fs;

use crate::scanner::Scanner;

pub fn eval_file(path: &str) {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let _ = eval(&contents);
}

pub fn eval(expr: &str) -> Vec<String> {
    let scanner = Scanner::new(expr);

    scanner.into_iter().map(|t| format!("{:?}", t)).collect()
}
