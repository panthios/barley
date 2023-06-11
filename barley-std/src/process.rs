use barley_runtime::prelude::*;
use tokio::process::Command as TokioCommand;

pub struct Command {
    command: Vec<ActionInput<String>>,
    check: Option<Vec<ActionInput<String>>>,
    undo: Option<Vec<ActionInput<String>>>
}

impl Command {
    pub fn new(command: Vec<ActionInput<String>>) -> Self {
        Self {
            command,
            check: None,
            undo: None
        }
    }

    pub fn check(&mut self, check: Vec<ActionInput<String>>) -> &mut Self {
        self.check = Some(check);
        self
    }

    pub fn undo(&mut self, undo: Vec<ActionInput<String>>) -> &mut Self {
        self.undo = Some(undo);
        self
    }
}

async fn resolve_argv(argv: &Vec<ActionInput<String>>, ctx: Runtime) -> Result<Vec<String>, ActionError> {
    let mut resolved = Vec::new();

    for arg in argv {
        resolved.push(match arg {
            ActionInput::Static(value) => value.clone(),
            ActionInput::Dynamic(output) => ctx.get_output(output.clone()).await
                .ok_or(ActionError::NoActionReturn)?
                .try_into()?
        });
    }

    Ok(resolved)
}

#[async_trait]
impl Action for Command {
    async fn probe(&self, runtime: Runtime) -> Result<Probe, ActionError> {
        let needs_run = match &self.check {
            Some(check) => {
                let argv = resolve_argv(check, runtime).await?;
                let name = argv.first().unwrap().clone();

                let status = TokioCommand::new(argv.first().unwrap())
                    .args(&argv.into_iter().skip(1).collect::<Vec<String>>())
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status()
                    .await
                    .map_err(|e| ActionError::ActionFailed(
                        format!("Internal spawn error: {}", e),
                        format!("Failed to spawn command: {}. This is a bug in the Barley engine.", name))
                    )?;
                
                !status.success()
            },
            None => true
        };

        Ok(Probe {
            needs_run,
            can_rollback: self.undo.is_some()
        })
    }

    async fn run(&self, runtime: Runtime, op: Operation) -> Result<Option<ActionOutput>, ActionError> {
        if matches!(op, Operation::Rollback) && self.undo.is_none() {
            return Err(ActionError::OperationNotSupported)
        }

        let argv = resolve_argv(match op {
            Operation::Perform => &self.command,
            Operation::Rollback => &self.undo.as_ref().unwrap()
        }, runtime).await?;

        let name = argv.first().unwrap().clone();

        let status = TokioCommand::new(argv.first().unwrap())
            .args(&argv.into_iter().skip(1).collect::<Vec<String>>())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await
            .map_err(|e| ActionError::ActionFailed(
                format!("Internal spawn error: {}", e),
                format!("Failed to spawn command: {}. This is a bug in the Barley engine.", name)
            ))?;
        
        if !status.success() {
            return Err(ActionError::ActionFailed(
                format!("Command exited with non-zero status code: {}", status.code().unwrap_or(1)),
                format!("Failed to run command: {}", name)
            ))
        } else {
            Ok(None)
        }
    }

    fn display_name(&self) -> String {
        format!("Command: {}", match self.command.first() {
            Some(ActionInput::Static(value)) => value,
            Some(ActionInput::Dynamic(output)) => "<dynamic>",
            None => "<empty>"
        })
    }
}