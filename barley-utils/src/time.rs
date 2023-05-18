use tokio::time::sleep;
pub use tokio::time::Duration;
use async_trait::async_trait;
use barley_runtime::*;


/// A timer.
/// 
/// This action does not track whether the timer has
/// already been run. See [this issue](https://github.com/panthios/barley/issues/1)
/// for more information.
#[barley_action]
#[derive(Default)]
pub struct Sleep {
  duration: Duration
}

impl Sleep {
  /// Create a new `Sleep` action.
  pub fn new(duration: Duration) -> Self {
    Self { duration, ..Default::default() }
  }
}

#[barley_action]
#[async_trait]
impl Action for Sleep {
  async fn check(&self, ctx: &mut Context) -> Result<bool> {
    if let Some(_) = ctx.get_local(self, "complete") {
      Ok(true)
    } else {
      Ok(false)
    }
  }

  async fn perform(&self, ctx: &mut Context) -> Result<()> {
    sleep(self.duration).await;
    ctx.set_local(self, "complete", "");
    Ok(())
  }

  async fn rollback(&self, _ctx: &mut Context) -> Result<()> {
    Ok(())
  }

  fn display_name(&self) -> String {
    format!("Sleep for {:?}", self.duration)
  }
}