pub mod rarc;

use crate::rarc::RarcCommand;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Complete,
    Rarc {
        #[command(subcommand)]
        command: RarcCommand,
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
            Self::Rarc { command } => command.run().await?,
        }

        Ok(())
    }
}
