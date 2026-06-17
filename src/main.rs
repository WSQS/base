use std::io;
mod ast;
mod parse;
mod scan;
mod util;
mod value;
use parse::parse;
mod eval;
use eval::eval_program;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error in read_line");
    let program = parse(&input);
    eval_program(&program);
}
