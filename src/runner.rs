use crate::{interpreter::Interpret, parser::Parser, scanner::Scanner};
use std::fs;

pub fn eval_file(path: &str) {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let _ = eval(&contents);
}

pub fn eval(expr: &str) -> Vec<String> {
    let scanner = Scanner::new(expr);
    let mut parser = Parser::new(scanner);

    let ast = parser.parse();

    match ast {
        Ok(stmts) => stmts
            .into_iter()
            .map(|t| t.evaluate())
            .map(|r| match r {
                Ok(l) => l.to_string(),
                Err(err) => err.to_string(),
            })
            .collect(),
        Err(err) => {
            vec![err.to_string()]
        }
    }
}
