use std::process::ExitCode;

use clap::Parser;

mod cli;

fn main() -> ExitCode {
    let args = cli::Args::parse();

    if let Err(s) = args.command.run() {
        eprintln!("{}", s);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
