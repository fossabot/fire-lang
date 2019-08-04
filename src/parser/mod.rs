mod lexer;

use lexer::{Token, lex};
use std::fs::read_to_string;
use std::process::exit;

struct Parser {
    filename: String,
    lines: Vec<String>,
    tokens: Vec<Token>,
    token: Token,
    token_i: usize,
    errors: usize,
    line: usize
}

impl Parser {
    fn new(filename: String) -> Self {
        let src = match read_to_string(&filename) {
            Err(e) => panic!("{}", e),
            Ok(s) => s,
        };

        let lines = src.lines()
            .map(|s| s.to_string())
            .collect();

        Parser {
            lines,
            filename,
            tokens: lex(src),
            token: Token {
                ttype: "".to_string(),
                value: "".to_string()
            },
            token_i: 0,
            errors: 0,
            line: 0
        }
    }

    fn next(&mut self) {
        self.token_i += 1;
        let token = &self.tokens[self.token_i-1];
        self.token = Token {
            value: token.value.clone(),
            ttype: token.ttype.clone()
        };
    }

    fn function(&mut self, extern_function: bool) -> String {
        /* Convert fire function to C
         * `fn FuncName(arg1: Type, arg2: Type) -> RetType`
         * =>
         * `RetType __fire_FuncName(Type arg1, Type arg2)`
         */
        self.next();
        let fname = format!("{}{}", if !extern_function { "__fire_" } else { "" }, self.token.value);

        /* after `fn` the function name is required */
        if !self.see("Name") {
            self.errors += 1;
            self.error("invalid syntax", "expected name");
        }

        /* default return type: void */
        let mut ftype = "void".to_string();
        self.next();

        if !self.see_value("(") {
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

        while !self.see_value(")") {
            self.next();

            if self.see_value("...") {
                args = format!("{}...", args);
                self.next();
                break;
            }

            if self.see_value(":") || self.see_value(",") {
                if self.see_value(",") {
                    args = format!("{},", args);
                }
                continue;
            }

            if atype {
                args = format!("{}{} {}", args, self.token.value, aname);
            } else {
                aname = format!("__fire_{}", self.token.value);
            }

            atype = !atype;
        }

        if &self.tokens[self.token_i+1].ttype == "Arrow" {
            self.next(); // point to arrow
            self.next(); // skip `->` and get type
            ftype = self.token.value.clone();
        }

        format!("{} {}({})", ftype, fname, args)
    }

    fn variable(&mut self) -> String {
        /* Convert fire variable to C
         * `let var: Type = ...`
         * =>
         * `const Type var = ...`
         *
         * `let mut var: Type = ...`
         * =>
         * `Type var = ...`
         */
        self.next();
        let name = self.token.value.clone();
        let mut var_type = None;

        /* after `let` the variable name is required */
        if !self.see("Name") {
            self.errors += 1;
            self.error("invalid syntax", "expected name");
        }

        self.next();
        if self.see_value(":") {
            self.next();
            var_type = Some(self.token.value.clone());

            /* after `:` the type name is required */
            if !self.see("Name") {
                self.errors += 1;
                self.error("invalid syntax", "expected type");
            }
        } else if !self.see_value("=") {
            self.error("invalid syntax", format!("unexpected {:?}", self.token).as_ref());
            self.errors += 1;
        }

        /* if type is not specified, we must deduce it from the value */
        if var_type == None {
            let token = &self.tokens[self.token_i];
            self.token_i -= 1;
            if token.ttype == "Int".to_string() {
                var_type = Some("long".to_string());
            } else if token.ttype == "String".to_string() {
                var_type = Some("char*".to_string());
            } else {
                self.error("unrecognized type", format!("cannot determine a type of variable `{}`", name).as_ref());
                self.errors += 1;
            }
        }

        format!("{} __fire_{}", var_type.unwrap_or("_".to_string()), name)
    }

    fn see(&self, s: &str) -> bool {
        self.token.ttype == s.to_string()
    }

    fn see_value(&self, s: &str) -> bool {
        self.token.value == s.to_string()
    }

    fn parse(&mut self) -> String {
        let mut output = String::new();

        while self.token_i < self.tokens.len() {
            self.next();

            if self.see("Fn") {
                output = format!("{}\n{}", output, self.function(false));
            }

            else if self.see("Extern") {
                self.next();
                output = format!("{}\n{}", output, self.function(true));
            }

            else if self.see("Let") {
                output = format!("{}\n{}", output, self.variable());
            }

            else if self.see("Newline") {
                let line = &self.lines[self.line];
                self.line += 1;
                output = format!("{}\n//{}:{}@{}\n", output, self.filename, self.line, line);
            }

            else {
                output = format!("{}{}", output, self.token.value);
            }
        }

        output
    }

    // TODO: pretty displaying of errors
    fn error<'a>(&self, t: &'a str, e: &'a str) {
        panic!("{}: {}: {}", self.filename, t, e);
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
