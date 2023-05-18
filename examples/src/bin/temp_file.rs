use barley_interface::Interface;
use barley_runtime::*;
use barley_utils::fs::{TempFile};


#[tokio::main]
async fn main() -> Result<()> {
  let mut interface = Interface::new();

  let temp_file = TempFile::new("temp_file_test".to_string());
  interface.add_action(temp_file);

  interface.run().await
}