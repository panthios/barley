use barley_runtime::*;
use barley_utils::time::*;
use barley_interface::*;


#[tokio::main]
async fn main() -> Result<()> {
  let mut interface = Interface::new();

  let wait_1 = Sleep::new(Duration::from_secs(1));
  let mut wait_2 = Sleep::new(Duration::from_secs(2));

  wait_2.add_dep(
    interface.add_action(wait_1)
  );

  interface.add_action(wait_2);

  
  interface.run().await
}