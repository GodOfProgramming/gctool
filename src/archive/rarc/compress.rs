use async_std::{fs, path::Path, stream::StreamExt};
use cube_rs::{Encode, rarc::Rarc, virtual_fs::VirtualFile};
use tokio::task::JoinSet;

pub async fn run(
    dir: impl AsRef<Path>,
    output: impl AsRef<Path>,
    all: bool,
    extension: Option<&str>,
) -> anyhow::Result<()> {
    if all {
        let mut iter = async_std::fs::read_dir(dir).await?;
        let mut set = JoinSet::new();
        let extension = extension.unwrap_or("szp");

        while let Some(res) = iter.next().await {
            let entry = res?;
            let path = entry.path();

            if let Some(filename) = path.file_name() {
                let output = output.as_ref().join(filename).with_extension(extension);
                set.spawn(compress_dir(path, output));
            }
        }

        while let Some(res) = set.join_next().await {
            let _ = res?;
        }

        Ok(())
    } else {
        compress_dir(dir, output).await
    }
}

pub async fn compress_dir(dir: impl AsRef<Path>, output: impl AsRef<Path>) -> anyhow::Result<()> {
    let vfile = compress(dir)?;

    if let Some(parent) = output.as_ref().parent() {
        fs::create_dir_all(parent).await?;
    }

    fs::write(output, vfile.bytes).await?;

    Ok(())
}

pub fn compress(dir: impl AsRef<Path>) -> anyhow::Result<VirtualFile> {
    let vfile = Rarc::encode(std::path::Path::new(dir.as_ref()))?;
    Ok(vfile)
}
