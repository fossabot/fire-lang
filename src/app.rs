use clap::{App, Arg};

pub struct Args {
    pub file: String,
    pub output: String
}

impl Args {
    pub fn parse() -> Args {
        let matches = App::new("fire")
            .version("0.1.0")
            .about("Compiler for fire programming language")
            .author("maviek")
            .arg(Arg::with_name("FILE")
                .help("Source file to compile")
                .required(true)
                .index(1))
            .arg(Arg::with_name("OUTPUT")
                .short("-o")
                .long("--output")
                .takes_value(true)
                .help("Output path"))
            .get_matches();
        
        Args {
            file: matches.value_of("FILE").unwrap().to_string(),
            output: matches.value_of("OUTPUT").unwrap_or("a").to_string(),
        }
    }
}
