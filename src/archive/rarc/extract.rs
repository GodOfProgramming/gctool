use async_std::{
    fs,
    path::{Path, PathBuf},
    stream::StreamExt,
};
use cube_rs::rarc::Rarc;
use tokio::task::JoinSet;

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
                set.spawn(extract_file(path, output.as_ref().join(basename)));
            }
        }

        while let Some(res) = set.join_next().await {
            let _ = res?;
        }

        Ok(())
    } else {
        extract_file(path, output).await
    }
}

pub async fn extract_file(path: impl AsRef<Path>, output: impl AsRef<Path>) -> anyhow::Result<()> {
    let data = fs::read(path).await?;
    let rarc = extract(&data)?;

    for (i, (filename, data)) in rarc.files().enumerate() {
        println!("Writing file {i} => {}", filename.display());

        let filepath = output.as_ref().join(filename);

        if let Some(p) = filepath.parent() {
            fs::create_dir_all(p).await?;
        }

        fs::write(filepath, data).await?;
    }

    Ok(())
}

pub fn extract<'d, D>(data: &'d D) -> anyhow::Result<Rarc<'d>>
where
    D: 'd + AsRef<[u8]>,
{
    let rarc = Rarc::parse(data.as_ref())?;
    Ok(rarc)
}
