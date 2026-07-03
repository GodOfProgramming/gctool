use async_std::{
    fs,
    path::{Path, PathBuf},
    stream::StreamExt,
};
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

            if path.is_file().await
                && let Some(filename) = path.file_name()
                && let Some(basename) = PathBuf::from(&filename).file_stem()
            {
                set.spawn(decompress_file(path, output.as_ref().join(basename)));
            }
        }

        while let Some(res) = set.join_next().await {
            let _ = res?;
        }

        Ok(())
    } else {
        decompress_file(path, output).await
    }
}

pub async fn decompress_file(
    path: impl AsRef<Path>,
    output: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let data = fs::read(path).await?;
    let data = decompress(data)?;
    util::ensure_parent(&output).await?;
    fs::write(output, data).await?;
    Ok(())
}

pub fn decompress(data: impl AsRef<[u8]>) -> anyhow::Result<Vec<u8>> {
    let data = szs::decode_yay0(data.as_ref())?;
    Ok(data)
}
