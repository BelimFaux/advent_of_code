use std::{env, process};

const _YEAR: u32 = 2025;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = args.get(1).unwrap_or_else(|| {
        eprintln!("Expected <day> as first argument");
        process::exit(64);
    });

    let _session_token = env::var("SESSION_TOKEN").unwrap_or_else(|_| {
        eprintln!(
            "Couldn't find a session token. Make sure a SESSION_TOKEN environment variable exists."
        );
        process::exit(64);
    });

    println!("Downloading day {}", day);
}
