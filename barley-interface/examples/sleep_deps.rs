use barley_interface::*;
use barley_runtime::*;
use barley_utils::time::{Duration, Sleep};


#[tokio::main]
async fn main() -> Result<()> {
  let timer_1 = Sleep::new(Duration::from_secs(1));
  let mut timer_2 = Sleep::new(Duration::from_secs(2));

  let interface = Interface::new();

  timer_2.add_dep(
    interface.add_action(timer_1).await
  );

  interface.add_action(timer_2).await;

  interface.run().await
}