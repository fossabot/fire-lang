mod app;
mod parser;

use std::process::Command;
use std::fs::{File, remove_file};
use std::io::Write;
use std::str::from_utf8;

fn main() {
    let args = app::Args::parse();
    let output = parser::compile(args.file);
    let builtins = parser::compile_string(include_str!("builtins.fr").to_string());
    let filename = "/tmp/__fire.cc";

    match File::create(&filename) {
        Ok(mut file) => {
            file.write(format!("{}\n{}", builtins, output).as_bytes()).unwrap();
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

    println!("{}", from_utf8(&cmd.stderr).unwrap());
    remove_file(filename).unwrap();

    if args.run {
        let filename = format!("./{}", args.output);
        match Command::new(&filename).spawn() {
            Ok(mut cmd) => {
                cmd.wait().unwrap();
                remove_file(filename).unwrap();
            }
            _ => ()
        };
    }
}
