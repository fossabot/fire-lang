mod app;
mod parser;

use std::process::Command;
use std::fs::{File, remove_file};
use std::io::Write;
use std::str::from_utf8;

fn main() {
    let args = app::Args::parse();
    let output = parser::compile(args.file.clone());
    println!("{}", output);

    let filename = format!("/tmp/{}.c", args.file.replace("/", "_").replace("\\", "_"));

    match File::create(&filename) {
        Ok(mut file) => {
            file.write(output.as_bytes()).unwrap();

            let cmd = Command::new("cc")
                .arg(&filename)
                .arg("-o")
                .arg(&args.output)
                .output()
                .expect("failed to execute process");

            println!("{}", from_utf8(&cmd.stderr).unwrap());

            remove_file(filename).unwrap();
        },
        Err(e) => panic!("{}", e)
    }

    let filename = format!("./{}", args.output);
    let mut cmd = Command::new(&filename)
        .spawn()
        .expect("failed to execute process");

    cmd.wait().unwrap();
    remove_file(filename).unwrap();
}
