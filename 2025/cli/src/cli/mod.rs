use clap::{Parser, Subcommand};

mod download;
mod scaffold;
mod util;

const YEAR: u32 = 2024; // set to 2024 for debugging; 2025 will be available in december

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
}

impl Command {
    pub fn run(self) -> Result<(), String> {
        match self {
            Command::Download { day } => download::download(day),
            Command::Scaffold { day } => scaffold::scaffold(day),
        }?;
        Ok(())
    }
}
