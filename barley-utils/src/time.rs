use tokio::time::sleep;
pub use tokio::time::Duration;
use async_trait::async_trait;
use barley_runtime::*;


#[barley_action]
#[derive(Default)]
pub struct Sleep {
  duration: Duration
}

impl Sleep {
  pub fn new(duration: Duration) -> Self {
    Self { duration, ..Default::default() }
  }
}

#[barley_action]
#[async_trait]
impl Action for Sleep {
  async fn check(&self, ctx: &mut Context) -> Result<bool> {
    Ok(false)
  }

  async fn perform(&self, ctx: &mut Context) -> Result<()> {
    sleep(self.duration).await;
    Ok(())
  }
}