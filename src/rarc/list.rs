use async_std::{fs, path::Path};
use cube_rs::rarc::Rarc;
use itertools::Itertools;

pub async fn run(path: &Path) -> anyhow::Result<()> {
    let data = fs::read(path).await?;
    let rarc = Rarc::parse(&data)?;

    for (filename, _) in rarc.files().sorted_by(|a, b| a.0.cmp(&b.0)) {
        println!("{}", filename.display());
    }

    Ok(())
}
