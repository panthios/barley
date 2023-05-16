use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;
use barley_runtime::{Action, Result, Context, barley_action};
use async_trait::async_trait;
use std::path::PathBuf;


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

#[async_trait]
impl Action for FileW {
  async fn check(&self, _ctx: &mut Context) -> Result<bool> {
    let path = PathBuf::from(&self.path);
    Ok(path.exists())
  }

  async fn perform(&self, _ctx: &mut Context) -> Result<()> {
    let mut file = TokioFile::create(&self.path).await?;
    file.write_all(self.content.as_bytes()).await?;

    Ok(())
  }
}