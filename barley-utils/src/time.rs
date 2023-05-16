use tokio::time::sleep;
pub use tokio::time::Duration;
use barley_runtime::{Action, Result, Context};
use async_trait::async_trait;


pub struct Sleep {
  duration: Duration
}

impl Sleep {
  pub fn new(duration: Duration) -> Self {
    Self { duration }
  }
}

#[async_trait]
impl Action for Sleep {
  async fn check(&self, _ctx: &mut Context) -> Result<bool> {
    Ok(false)
  }

  async fn perform(&self, _ctx: &mut Context) -> Result<()> {
    sleep(self.duration).await;
    Ok(())
  }
}