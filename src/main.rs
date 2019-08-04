mod app;
mod parser;

fn main() {
    let args = app::Args::parse();
    let output = parser::compile(args.file);
    println!("{}", output);
}
