pub mod compress;
pub mod decompress;

use async_std::path::PathBuf;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum CompressionCommand {
    Compress {
        input: PathBuf,
        output: PathBuf,
        #[arg(short, long)]
        all: bool,
    },
    Decompress {
        input: PathBuf,
        output: PathBuf,
        #[arg(short, long)]
        all: bool,
    },
}

impl CompressionCommand {
    pub async fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::Compress { input, output, all } => compress::run(input, output, *all).await?,
            Self::Decompress { input, output, all } => decompress::run(input, output, *all).await?,
        }

        Ok(())
    }
}
