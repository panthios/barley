use barley_runtime::prelude::*;
use barley_utils::time::{Sleep, Duration};



#[tokio::test]
async fn sleep() -> Result<()> {
  let context = Context::new(Default::default());
  context.clone().add_action(Sleep::new(Duration::from_millis(100))).await;

  context.run().await
}