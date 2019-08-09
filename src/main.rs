mod compiler;

use std::process::Command;
use std::fs::remove_file;
use compiler::app::Args;

fn main() {
    let args = Args::parse();
    compiler::compile(&args);

    if args.run {
        let filename = format!("./{}", args.output);
        match Command::new(&filename)
            .spawn() { Ok(mut cmd) => { cmd.wait().unwrap(); }, _ => () };
        remove_file(filename).unwrap_or_default();
    }
}
