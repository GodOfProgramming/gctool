mod compress;
mod extract;
mod list;

use async_std::path::PathBuf;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum RarcCommand {
    List {
        path: PathBuf,
    },
    Extract {
        input: PathBuf,
        outdir: PathBuf,
        #[arg(short, long)]
        all: bool,
    },
    Compress {
        indir: PathBuf,
        output: PathBuf,
        #[arg(short, long)]
        all: bool,
        #[arg(short, long)]
        extension: Option<String>,
    },
}

impl RarcCommand {
    pub async fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::List { path } => list::run(path).await?,
            Self::Extract { input, outdir, all } => extract::run(input, outdir, *all).await?,
            Self::Compress {
                indir,
                output,
                all,
                extension,
            } => compress::run(indir, output, *all, extension.as_ref().map(|e| e.as_str())).await?,
        }

        Ok(())
    }
}
