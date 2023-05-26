use barley_interface::Interface;
use barley_runtime::prelude::*;


#[tokio::main]
async fn main() -> Result<()> {
  let interface = Interface::new();


  interface.run().await
}