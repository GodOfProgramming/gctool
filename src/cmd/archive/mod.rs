pub mod rarc;

use clap::Subcommand;
use rarc::RarcCommand;

#[derive(Subcommand, Debug)]
pub enum ArchiveCommand {
    Rarc {
        #[command(subcommand)]
        command: RarcCommand,
    },
}

impl ArchiveCommand {
    pub async fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::Rarc { command } => command.run().await?,
        }

        Ok(())
    }
}
