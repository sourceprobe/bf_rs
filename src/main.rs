use bf_rs as crt;
use std::io::Write;

fn main() {
    let args 
        = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("usage: ./bf file.bf");
        return;
    }

    let program = std::fs::read(&args[1])
        .expect("Failed to read file.");
    let program = std::str::from_utf8(&program)
        .expect("Non-unicode program.");
    let program = crt::cleanup(&program);
    crt::run(&program);

}
