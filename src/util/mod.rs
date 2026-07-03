use async_std::{fs, path::Path};

pub async fn ensure_parent(path: impl AsRef<Path>) -> anyhow::Result<()> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).await?;
    }

    Ok(())
}
