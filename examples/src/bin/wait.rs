use barley_runtime::*;
use barley_utils::time::*;
use barley_interface::*;


#[tokio::main]
async fn main() -> Result<()> {
  let interface = Interface::new();

  let wait_1 = Sleep::new(Duration::from_secs(1));
  let mut wait_2 = Sleep::new(Duration::from_secs(2));
  let mut wait_3 = Sleep::new(Duration::from_secs(3));

  wait_2.add_dep(interface.add_action(wait_1).await);
  wait_3.add_dep(interface.add_action(wait_2).await);

  interface.add_action(wait_3).await;
  
  interface.run().await
}