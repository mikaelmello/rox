use std::fs;

pub fn eval_file(path: &str) {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let _ = eval(&contents);
}

pub fn eval(expr: &str) -> String {
    return expr.into();
}
