use std::{fs, io::Cursor};

use crate::{interpreter::Interpret, lexer::scanner::Scanner, parser::Parser};

pub fn eval_file(path: &str) {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let _ = eval(&contents);
}

pub fn eval(expr: &str) -> Vec<String> {
    let scanner = Scanner::from(Cursor::new(expr));
    let mut parser = Parser::new(scanner.into_iter());

    let ast = parser.parse();

    match ast {
        Ok(stmts) => {
            let mut results = vec![];
            for stmt in stmts {
                let result = stmt.evaluate();

                match result {
                    Ok(expr) => results.push(format!("{}", expr)),
                    Err(e) => {
                        results.push(format!("{}", e));
                        break;
                    }
                }
            }

            results
        }
        Err(e) => {
            vec![format!("{}", e)]
        }
    }
}
