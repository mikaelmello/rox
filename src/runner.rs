use std::{fs, io::Cursor};

use crate::{lexer::scanner::Scanner, parser::Parser};

pub fn eval_file(path: &str) {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let _ = eval(&contents);
}

pub fn eval(expr: &str) -> String {
    let scanner = Scanner::from(Cursor::new(expr));
    let mut parser = Parser::new(scanner.into_iter());

    let ast = parser.expression();

    if let Ok(expr) = ast {
        let result = expr.evaluate();

        match result {
            Ok(expr) => format!("{}", expr),
            Err(e) => format!("{}", e),
        }
    } else {
        format!("{}", ast.unwrap_err())
    }
}
