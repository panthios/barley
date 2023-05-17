use barley_runtime::{Context, Result};
use barley_utils::time::{Sleep, Duration};



#[tokio::test]
async fn sleep() -> Result<()> {
  let mut context = Context::new(Default::default());
  context.add_action(Sleep::new(Duration::from_millis(100)));

  context.run().await
}