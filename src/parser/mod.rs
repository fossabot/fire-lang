mod lexer;

use lexer::{Token, lex};
use std::fs::read_to_string;
use std::process::exit;

struct Parser {
    filename: String,
    src: String,
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
            Err(_) => "".to_string(),
            Ok(s) => s,
        };

        let lines = src.lines()
            .map(|s| s.to_string())
            .collect();

        Parser {
            lines,
            filename,
            tokens: lex(src.clone()),
            src,
            token: Token {
                ttype: "".to_string(),
                value: "".to_string()
            },
            token_i: 0,
            errors: 0,
            line: 0
        }
    }

    fn init(&mut self) {
        let lines = self.src.lines()
            .map(|s| s.to_string())
            .collect();

        self.lines = lines;
        self.tokens = lex(self.src.clone());
    }

    fn next(&mut self) {
        self.token_i += 1;
        let token = &self.tokens[self.token_i-1];
        self.token = Token {
            value: token.value.clone(),
            ttype: token.ttype.clone()
        };
    }

    fn function(&mut self) -> String {
        /* Convert fire function to C
         * `fn FuncName(arg1: Type, arg2: Type) -> RetType`
         * =>
         * `RetType __fire_FuncName(Type arg1, Type arg2)`
         */
        self.next();

        let mut template = String::new();

        // template: `fn<T>`
        if self.see_value("<") {
            self.next(); // skip `<`
            template = "template<typename".to_string();
            while !self.see_value(">") {
                if self.see("Name") {
                    template = format!("{} __fire_{}", template, self.token.value);
                } else if self.see_value("...") {
                    template = format!("{}...", template);
                } else if self.see_value(",") {
                    template = format!("{}, typename", template);
                } else {
                    self.errors += 1;
                    self.error("invalid syntax", format!("unexpected token `{}` in function template", self.token.value).as_str());
                }
                self.next();
            }
            template = format!("{}>", template);
            self.next(); // skip `>`
        }

        let fname = format!("__fire_{}", self.token.value);

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
        let mut amulti = false;

        while !self.see_value(")") {
            self.next();

            if self.see_value("...") {
                amulti = true;
                continue;
            }

            if self.see_value(":") || self.see_value(",") {
                if self.see_value(",") {
                    args = format!("{},", args);
                }
                continue;
            }

            if atype {
                args = format!("{}__fire_{}{} {}", args, self.token.value, if amulti { "..." } else { "" }, aname);
                amulti = false;
            } else {
                aname = format!("__fire_{}", self.token.value);
            }

            atype = !atype;
        }

        self.next(); // point to `->` or `{`
        if self.see("Arrow") {
            self.next(); // skip `->` and get type
            ftype = format!("__fire_{}", self.token.value);
            self.next();
            if !self.see_value("{") {
                self.errors += 1;
                self.error("invalid syntax", "expected `{` after function definition");
            }
        } else if !self.see_value("{") && !self.see_value(";") {
            self.errors += 1;
            self.error("invalid syntax", "expected `{` or `;` after function definition");
        }

        self.token_i -= 1;
        format!("{}\n{} {}({})", template, ftype, fname, args)
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
        let mut var_type = "auto".to_string();
        let mut b_mut = false;

        if self.see("Mut") {
            self.next();
            b_mut = true;
        }

        let name = self.token.value.clone();

        /* after `let` or `mut` the variable name is required */
        if !self.see("Name") {
            self.errors += 1;
            self.error("invalid syntax", format!("expected name got `{}`", self.token.value).as_str());
        }

        self.next();
        if self.see_value(":") {
            self.next();
            var_type = format!("__fire_{}", self.token.value);

            /* after `:` the type name is required */
            if !self.see("Name") {
                self.errors += 1;
                self.error("invalid syntax", "expected type");
            }

            self.next();
        } else if !self.see_value("=") {
            self.error("invalid syntax", format!("unexpected {:?}", self.token).as_ref());
            self.errors += 1;
        }

        format!("{}{} __fire_{}=", if b_mut { "" } else { "const " }, var_type, name)
    }

    fn see(&self, s: &str) -> bool {
        self.token.ttype == s.to_string()
    }

    fn see_value(&self, s: &str) -> bool {
        self.token.value == s.to_string()
    }

    fn parse(&mut self) -> String {
        let mut output = include_str!("template.cc").to_string();
        let mut close = false;

        while self.token_i < self.tokens.len() {
            self.next();

            if close && self.see_value("{") {
                output = format!("{})", output);
                close = false;
            }

            if self.see("Fn") {
                output = format!("{}\n{}", output, self.function());
            }

            else if self.see("Let") {
                output = format!("{}\n{}", output, self.variable());
            }

            else if self.see("Directive") {
                output = format!("{}\n{}\n", output, self.token.value);
            }

            else if self.see("Name") {
                output = format!("{}__fire_{}", output, self.token.value);
            }

            else if self.see("Loop") {
                output = format!("{}while(true)", output);
            }

            else if self.see("Return") {
                output = format!("{}{} ", output, self.token.value);
            }

            else if self.see("If") || self.see("While") {
                output = format!("{}{}(", output, self.token.value);
                close = true;
            }

            else if self.see("String") {
                output = format!("{}std::string({})", output, self.token.value);
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

pub fn compile_string(code: String) -> String {
    let mut parser = Parser::new("<string>".to_string());
    parser.src = code;
    parser.init();
    let output = parser.parse();
    if parser.errors != 0 {
        exit(parser.errors as i32);
    }
    output
}
