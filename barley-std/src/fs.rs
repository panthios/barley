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
    async fn probe(&self, _runtime: Runtime) -> Result<Probe, ActionError> {
        Ok(Probe {
            needs_run: true,
            can_rollback: false
        })
    }

    async fn run(&self, runtime: Runtime, op: Operation) -> Result<Option<ActionOutput>, ActionError> {
        if matches!(op, Operation::Rollback) {
            return Err(ActionError::OperationNotSupported)
        }

        let content = match self.content {
            ActionInput::Static(ref s) => s.clone(),
            ActionInput::Dynamic(ref obj) => {
                let output = runtime.get_output(obj.clone()).await
                    .ok_or(ActionError::ActionFailed(
                        "Failed to get output".to_string(),
                        "Failed to get output".to_string()
                    ))?;

                match output {
                    ActionOutput::String(s) => s,
                    _ => return Err(ActionError::ActionFailed(
                        "Output is not a string".to_string(),
                        "Output is not a string".to_string()
                    ))
                }
            }
        };

        let mut file = File::create(&self.path).await
            .map_err(|e| ActionError::ActionFailed(
                format!("Failed to create file: {}", e),
                format!("Failed to create file: {}", self.path.display())
            ))?;
        
        file.write_all(content.as_bytes()).await
            .map_err(|e| ActionError::ActionFailed(
                format!("Failed to write to file: {}", e),
                format!("Failed to write to file: {}", self.path.display())
            ))?;
        
        Ok(None)
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
    async fn probe(&self, _runtime: Runtime) -> Result<Probe, ActionError> {
        Ok(Probe {
            needs_run: true,
            can_rollback: false
        })
    }

    async fn run(&self, _runtime: Runtime, op: Operation) -> Result<Option<ActionOutput>, ActionError> {
        if matches!(op, Operation::Rollback) {
            return Err(ActionError::OperationNotSupported)
        }

        let content = tokio::fs::read_to_string(&self.path).await
            .map_err(|e| ActionError::ActionFailed(
                format!("Failed to read file: {}", e),
                format!("Failed to read file: {}", self.path.display())
            ))?;

        Ok(Some(ActionOutput::String(content)))
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
    async fn probe(&self, _runtime: Runtime) -> Result<Probe, ActionError> {
        Ok(Probe {
            needs_run: self.path.exists(),
            can_rollback: false
        })
    }

    async fn run(&self, _runtime: Runtime, op: Operation) -> Result<Option<ActionOutput>, ActionError> {
        if matches!(op, Operation::Rollback) {
            return Err(ActionError::OperationNotSupported)
        }

        tokio::fs::remove_file(&self.path).await
            .map_err(|e| ActionError::ActionFailed(
                format!("Failed to delete file: {}", e),
                format!("Failed to delete file: {}", self.path.display())
            ))?;

        Ok(None)
    }

    fn display_name(&self) -> String {
        format!("Delete file {}", self.path.display())
    }
}