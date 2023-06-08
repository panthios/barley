use barley_runtime::prelude::*;
use tokio::process::Command as TokioCommand;

pub struct Command {
    command: String,
    args: Vec<ActionInput<String>>
}

impl Command {
    pub fn new(command: String, args: Vec<ActionInput<String>>) -> Self {
        Self {
            command,
            args
        }
    }
}

#[async_trait]
impl Action for Command {
    async fn check(&self, _ctx: Runtime) -> Result<bool, ActionError> {
        Ok(false)
    }

    async fn perform(&self, ctx: Runtime) -> Result<Option<ActionOutput>, ActionError> {
        let mut command = TokioCommand::new(&self.command);
        for arg in &self.args {
            command.arg(match arg {
                ActionInput::Static(value) => value.clone(),
                ActionInput::Dynamic(output) => ctx.get_output(output.clone()).await
                    .ok_or(ActionError::NoActionReturn)?
                    .try_into()?
            });
        }

        let status = command.output().await
            .map_err(|e| ActionError::ActionFailed(
                format!("Failed to execute command: {}", e)
            ))?;

        if status.status.success() {
            Ok(None)
        } else {
            Err(ActionError::ActionFailed(
                format!("Command failed with status: {}", status.status)
            ))
        }
    }

    async fn rollback(&self, _ctx: Runtime) -> Result<(), ActionError> {
        Ok(())
    }

    fn display_name(&self) -> String {
        format!("command: {}", self.command)
    }
}