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
    async fn check(&self, _ctx: Runtime) -> Result<bool, ActionError> {
        Ok(false)
    }

    async fn perform(&self, _ctx: Runtime) -> Result<Option<ActionOutput>, ActionError> {
        sleep(self.duration).await;

        Ok(None)
    }

    async fn rollback(&self, _ctx: Runtime) -> Result<(), ActionError> {
        Ok(())
    }

    fn display_name(&self) -> String {
        format!("Sleep for {:?}", self.duration)
    }
}