use async_std::{fs, path::Path, stream::StreamExt};
use tokio::task::JoinSet;

use crate::util;

pub async fn run(
    path: impl AsRef<Path>,
    output: impl AsRef<Path>,
    all: bool,
) -> anyhow::Result<()> {
    if all {
        let mut iter = async_std::fs::read_dir(path).await?;
        let mut set = JoinSet::new();

        while let Some(res) = iter.next().await {
            let entry = res?;
            let path = entry.path();

            if let Some(filename) = path.file_name() {
                let output = output.as_ref().join(filename).with_extension("szp");
                let task = compress_file(path, output);
                set.spawn(task);
            }
        }

        while let Some(res) = set.join_next().await {
            res??;
        }
    } else {
        compress_file(path, output).await?;
    }

    Ok(())
}

pub async fn compress_file(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let data = fs::read(input).await?;
    let data = compress(data)?;
    util::ensure_parent(&output).await?;
    fs::write(output, data).await?;
    Ok(())
}

pub fn compress(data: impl AsRef<[u8]>) -> anyhow::Result<Vec<u8>> {
    let data = szs::encode_yay0(data.as_ref(), szs::EncodeAlgo::Nintendo)?;
    Ok(data)
}
