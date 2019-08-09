pub mod app;

mod error;
mod parser;

use app::Args;
use std::process::Command;
use std::fs::{File, remove_file};
use std::io::Write;
use std::str::from_utf8;

pub fn compile(args: &Args) {
    let output = parser::compile(args.file.clone());
    let builtins = parser::compile_string(include_str!("builtins.fr").to_string());
    let filename = "/tmp/__fire.cc";

    let cc_output = format!("{}\n{}", builtins, output);

    match File::create(&filename) {
        Ok(mut file) => {
            file.write(cc_output.as_bytes()).unwrap();
        },
        Err(e) => panic!("{}", e)
    }

    let cmd = Command::new("c++")
        .arg(&filename)
        .arg("-std=c++17")
        .arg("-fno-exceptions")
        .arg("-fno-rtti")
        .arg("-Ofast")
        .arg("-o")
        .arg(&args.output)
        .output()
        .expect("failed to execute process");

    error::display(cc_output, from_utf8(&cmd.stderr).unwrap());
    remove_file(filename).unwrap();
}
