use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;
use async_trait::async_trait;
use std::path::PathBuf;
use barley_runtime::*;


#[barley_action]
#[derive(Default)]
pub struct FileW {
  path: String,
  content: String
}

impl FileW {
  pub fn new(path: String, content: String) -> Self {
    Self { path, content, ..Default::default() }
  }
}

#[barley_action]
#[async_trait]
impl Action for FileW {
  async fn check(&self, ctx: &mut Context) -> Result<bool> {
    let path = PathBuf::from(&self.path);
    Ok(path.exists())
  }

  async fn perform(&self, _ctx: &mut Context) -> Result<()> {
    let mut file = TokioFile::create(&self.path).await?;
    file.write_all(self.content.as_bytes()).await?;

    Ok(())
  }
}


#[cfg(target_os = "windows")]
const ROOT_TEMP_DIR: &str = r"C:\Windows\Temp";
#[cfg(target_os = "linux")]
const ROOT_TEMP_DIR: &str = r"/tmp";

#[barley_action]
#[derive(Default)]
pub struct TempFile {
  rel_path: String
}

impl TempFile {
  pub fn new(rel_path: String) -> Self {
    Self { rel_path, ..Default::default() }
  }
}

#[barley_action]
#[async_trait]
impl Action for TempFile {
  async fn check(&self, ctx: &mut Context) -> Result<bool> {
    let path = PathBuf::from(ROOT_TEMP_DIR).join(&self.rel_path);
    Ok(path.exists())
  }

  async fn perform(&self, _ctx: &mut Context) -> Result<()> {
    let path = PathBuf::from(ROOT_TEMP_DIR).join(&self.rel_path);

    let mut file = TokioFile::create(&path).await?;
    file.write_all(b"").await?;

    Ok(())
  }
}