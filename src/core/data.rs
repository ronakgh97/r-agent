use anyhow::Result;
use std::path::PathBuf;
use tokio::fs;

pub async fn create_data_source() -> Result<PathBuf> {
    let home_dir =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    let source_path = home_dir.join(".config").join("r_agent").join("data");

    fs::create_dir_all(&source_path).await?;

    Ok(source_path)
}
