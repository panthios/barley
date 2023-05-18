use async_process::Command;
use async_trait::async_trait;
use barley_runtime::*;


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
  async fn check(&self, ctx: &mut Context) -> Result<bool> {
    if let Some(_) = ctx.get_local(self, "complete") {
      Ok(true)
    } else {
      Ok(false)
    }
  }

  async fn perform(&self, ctx: &mut Context) -> Result<Option<ActionOutput>> {
    let mut command = Command::new(&self.command[0]);
    command.args(&self.command[1..]);

    let output = command.output().await?;

    if output.status.success() {
      ctx.set_local(self, "complete", "");
      Ok(None)
    } else {
      Err(anyhow::anyhow!("Process failed"))
    }
  }

  async fn rollback(&self, _ctx: &mut Context) -> Result<()> {
    Ok(())
  }

  fn display_name(&self) -> String {
    format!("Shell: {}", &self.command[0])
  }
}