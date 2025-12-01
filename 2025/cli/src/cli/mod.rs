use clap::{Parser, Subcommand};

mod download;
mod scaffold;
mod solve;
mod util;

const YEAR: u32 = 2025;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Download {
        #[arg(required = true)]
        day: u8,
    },

    Scaffold {
        #[arg(required = true)]
        day: u8,
    },

    Solve {
        #[arg(required = true)]
        day: u8,
        #[arg(short, long)]
        /// run in release mode
        release: bool,
        #[arg(short, long)]
        /// only run tests
        test: bool,
    },
}

impl Command {
    pub fn run(self) -> Result<(), String> {
        match self {
            Command::Download { day } => download::download(day),
            Command::Scaffold { day } => scaffold::scaffold(day),
            Command::Solve { day, release, test } => solve::solve(day, release, test),
        }?;
        Ok(())
    }
}
