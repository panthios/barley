use barley_interface::Interface;
use barley_runtime::*;
use barley_utils::process::Process;
use barley_utils::fs::{TempFile, FileR};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
  let interface = Interface::new();

  let software_properties_common = Process::new(vec![
    "sudo".to_string(),
    "apt-get".to_string(),
    "install".to_string(),
    "-y".to_string(),
    "software-properties-common".to_string()
  ]);

  let mut ppa_deadsnakes = Process::new(vec![
    "sudo".to_string(),
    "add-apt-repository".to_string(),
    "-y".to_string(),
    "ppa:deadsnakes/ppa".to_string()
  ]);

  let mut apt_update = Process::new(vec![
    "sudo".to_string(),
    "apt-get".to_string(),
    "update".to_string()
  ]);

  let mut python39 = Process::new(vec![
    "sudo".to_string(),
    "apt-get".to_string(),
    "install".to_string(),
    "-y".to_string(),
    "python3.9".to_string()
  ]);

  let temp_file = TempFile::new("python_temp".to_string());
  let path = temp_file.path().to_str().unwrap().to_string();

  let script_path = env::current_dir().unwrap()
    .join("examples")
    .join("assets")
    .join("python_script.py");

  let mut call_python39 = Process::new(vec![
    "sudo".to_string(),
    "python3.9".to_string(),
    script_path.to_str().unwrap().to_string(),
    path.clone()
  ]);

  let mut read_file = FileR::new(path);

  ppa_deadsnakes.add_dep(
    interface.add_action(software_properties_common).await
  );

  apt_update.add_dep(
    interface.add_action(ppa_deadsnakes).await
  );

  python39.add_dep(
    interface.add_action(apt_update).await
  );

  call_python39.add_dep(
    interface.add_action(python39).await
  );

  call_python39.add_dep(
    interface.add_action(temp_file).await
  );

  read_file.add_dep(
    interface.add_action(call_python39).await
  );

  let read_file = interface.add_action(read_file).await;


  interface.run().await?;

  let output = interface.get_output_arc(read_file).await.unwrap();

  if let ActionOutput::String(output) = output {
    assert_eq!(output, "Hello from Python script")
  } else {
    panic!("Output is not a string")
  }

  Ok(())
}