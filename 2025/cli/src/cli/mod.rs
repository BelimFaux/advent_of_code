use clap::{Parser, Subcommand};

mod download;
mod scaffold;

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
