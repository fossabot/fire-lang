mod app;
mod lexer;
mod parser;

use std::fs::read_to_string;

fn main() {
    let args = app::Args::parse();
    let src = match read_to_string(args.file) {
        Err(e) => panic!("{}", e),
        Ok(s) => s,
    };

    let tokens = lexer::lex(src);
    let output = parser::parse(tokens);
    println!("{}", output);
}
