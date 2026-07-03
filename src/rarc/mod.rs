mod list;

use async_std::path::PathBuf;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum RarcCommand {
    List { path: PathBuf },
}

impl RarcCommand {
    pub async fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::List { path } => list::run(path).await?,
        }

        Ok(())
    }
}
