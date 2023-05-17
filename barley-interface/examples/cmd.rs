use barley_runtime::*;
use barley_utils::process::Process;
use barley_interface::Interface;


#[tokio::main]
async fn main() -> Result<()> {
  let mut interface = Interface::new();

  let process = Process::new(vec!["echo".to_string(), "Hello, world!".to_string()]);
  interface.add_action(process);

  interface.run().await
}