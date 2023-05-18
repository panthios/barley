use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;
use async_trait::async_trait;
use std::path::PathBuf;
use barley_runtime::*;

/// A writable file.
/// 
/// The content written is fixed. This will
/// be changed in the future.
#[barley_action]
#[derive(Default)]
pub struct FileW {
  path: String,
  content: String
}

impl FileW {
  /// Create a new `FileW` action.
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

  async fn perform(&self, _ctx: &mut Context) -> Result<Option<ActionOutput>> {
    let mut file = TokioFile::create(&self.path).await?;
    file.write_all(self.content.as_bytes()).await?;

    Ok(None)
  }

  async fn rollback(&self, _ctx: &mut Context) -> Result<()> {
    let path = PathBuf::from(&self.path);
    tokio::fs::remove_file(path).await?;

    Ok(())
  }

  fn display_name(&self) -> String {
    format!("Write data to \"{}\"", self.path)
  }
}


#[cfg(target_os = "windows")]
const ROOT_TEMP_DIR: &str = r"C:\Windows\Temp";
#[cfg(target_os = "linux")]
const ROOT_TEMP_DIR: &str = r"/tmp";

/// A temporary file.
///
/// This file does not currently have any write
/// access. This will be updated in the future.
#[barley_action]
#[derive(Default)]
pub struct TempFile {
  rel_path: String
}

impl TempFile {
  /// Create a new `TempFile` action.
  pub fn new(rel_path: String) -> Self {
    Self { rel_path, ..Default::default() }
  }

  /// Get the path of the temporary file.
  pub fn path(&self) -> PathBuf {
    PathBuf::from(ROOT_TEMP_DIR).join(&self.rel_path)
  }
}

#[barley_action]
#[async_trait]
impl Action for TempFile {
  async fn check(&self, ctx: &mut Context) -> Result<bool> {
    let path = PathBuf::from(ROOT_TEMP_DIR).join(&self.rel_path);
    Ok(path.exists())
  }

  async fn perform(&self, _ctx: &mut Context) -> Result<Option<ActionOutput>> {
    let path = PathBuf::from(ROOT_TEMP_DIR).join(&self.rel_path);

    let mut file = TokioFile::create(&path).await?;
    file.write_all(b"").await?;

    Ok(None)
  }

  async fn rollback(&self, _ctx: &mut Context) -> Result<()> {
    let path = PathBuf::from(ROOT_TEMP_DIR).join(&self.rel_path);
    tokio::fs::remove_file(path).await?;

    Ok(())
  }

  fn display_name(&self) -> String {
    format!("Create temporary file \"{}\"", self.rel_path)
  }
}