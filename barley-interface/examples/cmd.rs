use barley_runtime::*;
use barley_utils::process::Process;
use barley_interface::Interface;


#[tokio::main]
async fn main() -> Result<()> {
  let interface = Interface::new();

  let process = Process::new(vec!["echo".to_string(), "Hello, world!".to_string()]);
  interface.add_action(process).await;

  interface.run().await
}