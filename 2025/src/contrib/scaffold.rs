use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = args.get(1).unwrap_or_else(|| {
        eprintln!("Expected <day> as first argument");
        process::exit(64);
    });

    println!("Scaffolding day {}...", day);
}
