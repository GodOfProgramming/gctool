use clap::Parser;
use gctool::cmd::Args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    args.command.run(env!("CARGO_BIN_NAME")).await
}
