use anyhow::anyhow;
use barley_runtime::prelude::*;
use tokio::{fs::File, io::AsyncWriteExt};
use std::path::PathBuf;


pub struct WriteFile {
    path: PathBuf,
    content: ActionInput<String>
}

impl WriteFile {
    pub fn new_static<P, S>(path: P, content: S) -> Self
    where
        P: Into<PathBuf>,
        S: ToString,
    {
        Self {
            path: path.into(),
            content: ActionInput::new_static(content.to_string()),
        }
    }

    pub fn new_dynamic<P>(path: P, content: ActionObject) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            path: path.into(),
            content: ActionInput::new_dynamic(content)
        }
    }
}

#[async_trait]
impl Action for WriteFile {
    async fn check(&self, _ctx: Runtime) -> Result<bool> {
        Ok(false)
    }

    async fn perform(&self, ctx: Runtime) -> Result<Option<ActionOutput>> {
        let mut file = File::create(&self.path).await?;
        let content = match self.content {
            ActionInput::Static(ref s) => s.clone(),
            ActionInput::Dynamic(ref a) => {
                let output = ctx.get_output(a.clone()).await;

                if output.is_none() {
                    return Err(anyhow!("Action has no output"));
                }

                let output = output.unwrap();

                output.try_into()?
            }
        };

        file.write_all(content.as_bytes()).await?;

        Ok(None)
    }

    async fn rollback(&self, _ctx: Runtime) -> Result<()> {
        if self.path.exists() {
            tokio::fs::remove_file(&self.path).await?;
        }

        Ok(())
    }

    fn display_name(&self) -> String {
        format!("Write file {}", self.path.display())
    }
}

pub struct ReadFile {
    path: PathBuf
}

impl ReadFile {
    pub fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            path: path.into()
        }
    }
}

#[async_trait]
impl Action for ReadFile {
    async fn check(&self, _ctx: Runtime) -> Result<bool> {
        Ok(false)
    }

    async fn perform(&self, _ctx: Runtime) -> Result<Option<ActionOutput>> {
        let content = tokio::fs::read_to_string(&self.path).await?;

        Ok(Some(ActionOutput::String(content)))
    }

    async fn rollback(&self, _ctx: Runtime) -> Result<()> {
        Ok(())
    }

    fn display_name(&self) -> String {
        format!("Read file {}", self.path.display())
    }
}

pub struct DeleteFile {
    path: PathBuf
}

impl DeleteFile {
    pub fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            path: path.into()
        }
    }
}

#[async_trait]
impl Action for DeleteFile {
    async fn check(&self, _ctx: Runtime) -> Result<bool> {
        Ok(false)
    }

    async fn perform(&self, _ctx: Runtime) -> Result<Option<ActionOutput>> {
        tokio::fs::remove_file(&self.path).await?;

        Ok(None)
    }

    async fn rollback(&self, _ctx: Runtime) -> Result<()> {
        Ok(())
    }

    fn display_name(&self) -> String {
        format!("Delete file {}", self.path.display())
    }
}