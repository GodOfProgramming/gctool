pub mod cmd;
pub mod dol;
pub mod gcm;
pub mod util;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use cmd::{archive::ArchiveCommand, compression::CompressionCommand};
use thiserror::Error;

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Complete,
    Compression {
        #[command(subcommand)]
        command: CompressionCommand,
    },
    Archive {
        #[command(subcommand)]
        command: ArchiveCommand,
    },
}

impl Command {
    pub async fn run(&self, tool_name: &str) -> anyhow::Result<()> {
        match self {
            Self::Complete => clap_complete::generate(
                Shell::Bash,
                &mut Args::command(),
                tool_name,
                &mut std::io::stdout(),
            ),
            Self::Compression { command } => command.run().await?,
            Self::Archive { command } => command.run().await?,
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum GcError {
    #[error("Could not read {0}-{1} bytes into data")]
    OutOfRange(usize, usize),

    #[error("Generic Error: {0}")]
    Generic(String),
}

impl GcError {
    pub fn generic(s: impl ToString) -> Self {
        Self::Generic(s.to_string())
    }
}
