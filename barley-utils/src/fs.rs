use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;
use barley_runtime::{Action, Result};
use async_trait::async_trait;
use std::path::PathBuf;


pub struct FileW {
  path: String,
  content: String
}

impl FileW {
  pub fn new(path: String, content: String) -> Self {
    Self { path, content }
  }
}

#[async_trait]
impl Action for FileW {
  async fn check(&self) -> Result<bool> {
    let path = PathBuf::from(&self.path);
    Ok(path.exists())
  }

  async fn perform(&self) -> Result<()> {
    let mut file = TokioFile::create(&self.path).await?;
    file.write_all(self.content.as_bytes()).await?;

    Ok(())
  }
}