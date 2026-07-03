pub mod cmd;
pub mod util;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use cmd::{archive::ArchiveCommand, compression::CompressionCommand};

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
