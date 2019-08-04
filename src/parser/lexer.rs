use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum FireLexer {
    #[end] End,
    #[error] Error,
    #[regex = "\\{|\\}|\\(|\\)|\\[|\\]|\\+|-|\\*|/|%"] Literals,
    #[regex = "->"] Arrow,
    #[regex = "\\d+"] Number,
    #[regex = "\"(\\\\\"|[^\"])*\""] String,
    #[regex = "[_a-zA-Z][_a-zA-Z0-9]*"] Name,
    #[token = "break"] Break,
    #[token = "continue"] Continue,
    #[token = "else"] Else,
    #[token = "enum"] Enum,
    #[token = "false"] False,
    #[token = "fn"] Fn,
    #[token = "for"] For,
    #[token = "if"] If,
    #[token = "in"] In,
    #[token = "let"] Let,
    #[token = "loop"] Loop,
    #[token = "match"] Match,
    #[token = "mut"] Mut,
    #[token = "return"] Return,
    #[token = "static"] Static,
    #[token = "struct"] Struct,
    #[token = "true"] True,
    #[token = "use"] Use,
    #[token = "while"] While
}

#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub ttype: String
}

pub fn lex(code: String) -> Vec<Token> {
    let mut lexer = FireLexer::lexer(code.as_str());
    let mut tokens = Vec::new();

    loop {
        if lexer.token == FireLexer::End {
            break;
        }

        tokens.push(Token {
            value: lexer.slice().to_string(),
            ttype: format!("{:?}", lexer.token)
        });

        lexer.advance();
    }

    tokens
}
