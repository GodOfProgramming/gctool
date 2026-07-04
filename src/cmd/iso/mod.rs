pub mod list;

use async_std::path::PathBuf;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum IsoCommand {
    List {
        path: PathBuf,
        #[arg(short, long)]
        filter: Option<String>,
        #[arg(short, long)]
        regex: bool,
        #[arg(short, long)]
        insensitive: bool,
        #[arg(short, long)]
        pretty: bool,
    },
}

impl IsoCommand {
    pub async fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::List {
                path,
                filter,
                regex,
                insensitive,
                pretty,
            } => list::run(path, filter.as_deref(), *regex, *insensitive, *pretty).await?,
        }
        Ok(())
    }
}
