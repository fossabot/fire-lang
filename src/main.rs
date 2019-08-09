mod compiler;

use std::process::Command;
use std::fs::remove_file;
use compiler::app::Args;

fn main() {
    let args = Args::parse();
    compiler::compile(&args);

    if args.run {
        let filename = format!("./{}", args.output);
        Command::new(&filename)
            .spawn().expect("failed to run")
            .wait().unwrap();
        remove_file(filename).unwrap();
    }
}
