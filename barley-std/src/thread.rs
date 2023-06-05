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
    async fn check(&self, _ctx: Arc<RwLock<Context>>) -> Result<bool> {
        Ok(false)
    }

    async fn perform(&self, _ctx: Arc<RwLock<Context>>) -> Result<Option<ActionOutput>> {
        sleep(self.duration).await;

        Ok(None)
    }

    async fn rollback(&self, _ctx: Arc<RwLock<Context>>) -> Result<()> {
        Ok(())
    }

    fn display_name(&self) -> String {
        format!("Sleep for {:?}", self.duration)
    }
}