use barley_runtime::prelude::*;
use tokio::time::{sleep, Duration};



pub struct Sleep {
    duration: Duration,
}

impl Sleep {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration
        }
    }
}

#[async_trait]
impl Action for Sleep {
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

        sleep(self.duration).await;

        Ok(None)
    }

    fn display_name(&self) -> String {
        format!("Sleep for {} seconds", self.duration.as_secs())
    }
}