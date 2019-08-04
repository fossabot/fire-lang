mod lexer;

use lexer::{Token, lex};
use std::fs::read_to_string;
use std::process::exit;

struct Parser {
    filename: String,
    tokens: Vec<Token>,
    errors: usize
}

impl Parser {
    fn new(filename: String) -> Self {
        let src = match read_to_string(&filename) {
            Err(e) => panic!("{}", e),
            Ok(s) => s,
        };

        Parser {
            filename: filename,
            tokens: lex(src),
            errors: 0
        }
    }

    fn parse(&mut self) -> String {
        let mut output = String::new();
        let mut i = 0;

        while i < self.tokens.len() {
            let mut tok = &self.tokens[i];

            /* Convert fire function to C
            * `fn FuncName(arg1: Type, arg2: Type) -> RetType`
            * =>
            * `RetType __fire_FuncName(Type arg1, Type arg2)`
            */
            if tok.ttype == "Fn" {
                i += 1;
                tok = &self.tokens[i];

                let fname = format!("__fire_{}", tok.value);

                /* after `fn` the function name is required */
                if tok.ttype != "Name" {
                    self.errors += 1;
                    self.error("invalid syntax", "expected name");
                }

                /* default return type: void */
                let mut ftype = "void".to_string();

                i += 1;
                tok = &self.tokens[i];

                if tok.value != "(" {
                    self.errors += 1;
                    self.error("invalid syntax", "expected `(` after function name");
                }

                /* args -> output string with arguments in C style
                * aname -> current argument name
                * atype -> type flag, used to check if token is type or argument name
                */
                let mut args = String::new();
                let mut aname = String::new();
                let mut atype = false;

                while tok.value != ")" {
                    i += 1;
                    tok = &self.tokens[i];

                    if tok.value == ":" || tok.value == "," {
                        if tok.value == "," {
                            args = format!("{},", args);
                        }
                        continue;
                    }

                    if atype {
                        args = format!("{}{} {}", args, tok.value, aname);
                    } else {
                        aname = format!("__fire_{}", tok.value);
                    }

                    atype = !atype;
                }

                if &self.tokens[i+1].ttype == "Arrow" {
                    i += 2; // skip `->` and get type
                    ftype = self.tokens[i].value.clone();
                }

                output = format!("{}\n{} {}({})", output, ftype, fname, args);
            }

            else if tok.ttype == "Literals" {
                output = format!("{}{}", output, tok.value);
            }

            i += 1;
        }

        output
    }

    // TODO: pretty displaying of errors
    fn error<'a>(&self, t: &'a str, e: &'a str) {
        panic!("{}:{}: {}", self.filename, t, e);
    }
}

pub fn compile(filename: String) -> String {
    let mut parser = Parser::new(filename);
    let output = parser.parse();
    if parser.errors != 0 {
        exit(parser.errors as i32);
    }
    output
}
