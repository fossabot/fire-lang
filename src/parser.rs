use crate::lexer::Token;

fn error<'a>(t: &'a str, e: &'a str) {
    panic!("{}: {}", t, e);
}

pub fn parse(tokens: Vec<Token>) -> String {
    let mut output = String::new();
    let mut i = 0;

    while i < tokens.len() {
        let mut tok = &tokens[i];

        /* Convert fire function to C
         * `fn FuncName(arg1: Type, arg2: Type) -> RetType`
         * =>
         * `RetType __fire_FuncName(Type arg1, Type arg2)`
         */
        if tok.ttype == "Fn" {
            i += 1;
            tok = &tokens[i];

            let fname = format!("__fire_{}", tok.value);

            /* after `fn` the function name is required */
            if tok.ttype != "Name" {
                error("invalid syntax", "expected name");
            }

            /* default return type: void */
            let mut ftype = "void".to_string();

            i += 1;
            tok = &tokens[i];

            if tok.value != "(" {
                error("invalid syntax", "expected `(` after function name");
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
                tok = &tokens[i];

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

            if &tokens[i+1].ttype == "Arrow" {
                i += 2; // skip `->` and get type
                ftype = tokens[i].value.clone();
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
