use barley_interface::Interface;
use barley_runtime::prelude::*;
use barley_utils::fs::{FileW, FileR, TempFile};


#[tokio::main]
async fn main() -> Result<()> {
  let interface = Interface::new();

  let temp_file = TempFile::new("temp_file_test".to_string());
  let path = temp_file.path().to_str().unwrap().to_string();

  let mut write = FileW::new(path.clone(), "Hello, world!".to_string());
  let mut read = FileR::new(path.clone());

  write.add_dep(interface.add_action(temp_file).await);
  read.add_dep(interface.add_action(write).await);
  
  let read = interface.add_action(read).await;

  interface.run().await?;

  let output = interface.get_output_arc(read).await.unwrap();

  if let ActionOutput::String(content) = output {
    assert_eq!(content, "Hello, world!");
    println!("Output was \"{}\"", content);
  } else {
    panic!("Output was not a string!");
  }

  Ok(())
}