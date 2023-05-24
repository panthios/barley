use tokio::process::Command;
use async_trait::async_trait;
use barley_runtime::*;
use std::sync::Arc;
use tokio::sync::RwLock;


/// A command.
/// 
/// The output from the command is not captured, but the
/// status code is processed as a success or failure.
#[barley_action]
#[derive(Default)]
pub struct Process {
  command: Vec<String>
}

impl Process {
  /// Create a new `Process` action.
  pub fn new(command: Vec<String>) -> Self {
    Self { command, ..Default::default() }
  }
}

#[barley_action]
#[async_trait]
impl Action for Process {
  async fn check(&self, ctx: Arc<RwLock<Context>>) -> Result<bool> {
    if let Some(_) = ctx.get_local(self, "complete").await {
      Ok(true)
    } else {
      Ok(false)
    }
  }

  async fn perform(&self, ctx: Arc<RwLock<Context>>) -> Result<Option<ActionOutput>> {
    let mut command = Command::new(&self.command[0]);
    command.args(&self.command[1..]);

    let output = command.output().await?;

    if output.status.success() {
      ctx.set_local(self, "complete", "").await;
      Ok(None)
    } else {
      Err(anyhow::anyhow!("Process failed"))
    }
  }

  async fn rollback(&self, _ctx: Arc<RwLock<Context>>) -> Result<()> {
    Ok(())
  }

  fn display_name(&self) -> String {
    format!("Shell: {}", &self.command.join(" "))
  }
}

/// A command that captures its output.
/// 
/// This will only capture stdout. The status
/// code is converted to a success or failure.
#[barley_action]
#[derive(Default)]
pub struct ProcessWithOutput {
  command: Vec<String>
}

impl ProcessWithOutput {
  /// Create a new `ProcessWithOutput` action.
  pub fn new(command: Vec<String>) -> Self {
    Self { command, ..Default::default() }
  }
}

#[barley_action]
#[async_trait]
impl Action for ProcessWithOutput {
  async fn check(&self, ctx: Arc<RwLock<Context>>) -> Result<bool> {
    if let Some(_) = ctx.get_local(self, "complete").await {
      Ok(true)
    } else {
      Ok(false)
    }
  }

  async fn perform(&self, ctx: Arc<RwLock<Context>>) -> Result<Option<ActionOutput>> {
    let mut command = Command::new(&self.command[0]);
    command.args(&self.command[1..]);

    let output = command.output().await?;

    if output.status.success() {
      ctx.set_local(self, "complete", "").await;
      Ok(Some(ActionOutput::String(String::from_utf8(output.stdout)?)))
    } else {
      Err(anyhow::anyhow!("Process failed"))
    }
  }

  async fn rollback(&self, _ctx: Arc<RwLock<Context>>) -> Result<()> {
    Ok(())
  }

  fn display_name(&self) -> String {
    format!("Shell: {}", &self.command.join(" "))
  }
}