use barley_interface::Interface;
use barley_runtime::*;
use barley_utils::{
  time::{Duration, Sleep},
  Join
};


#[tokio::main]
async fn main() -> Result<()> {
  let interface = Interface::new();

  let sleep_1s = Sleep::new(Duration::from_secs(1));
  let sleep_2s = Sleep::new(Duration::from_secs(2));
  let sleep_3s = Sleep::new(Duration::from_secs(3));

  let join = Join::new(vec![
    interface.add_action(sleep_1s).await,
    interface.add_action(sleep_2s).await,
    interface.add_action(sleep_3s).await
  ]);

  interface.add_action(join).await;

  interface.run().await
}