use anyhow::{Result, anyhow};
use std::{
  env::current_dir,
  fs
};


pub fn cmd_init() -> Result<()> {
  let current_dir = current_dir()
    .or_else(|_| Err(anyhow!("Failed to get current directory")))?;

  let is_empty = {
    let mut entries = fs::read_dir(&current_dir)
      .or_else(|_| Err(anyhow!("Failed to read current directory")))?;

    entries.next().is_none()
  };

  if !is_empty {
    return Err(anyhow!("Current directory is not empty"));
  }

  fs::create_dir_all(&current_dir.join("src"))
    .or_else(|_| Err(anyhow!("Failed to create src directory")))?;

  fs::write(&current_dir.join("src/main.rs"), include_str!("../template/main.rs"))
    .or_else(|_| Err(anyhow!("Failed to create main.rs")))?;

  let cargo_toml: String = include_str!("../template/Cargo.toml")
    .replace("{{ name }}", format!("blyscript-{}", &current_dir.file_name().unwrap().to_string_lossy()).as_str());

  fs::write(&current_dir.join("Cargo.toml"), cargo_toml)
    .or_else(|_| Err(anyhow!("Failed to create Cargo.toml")))?;

  fs::write(&current_dir.join(".gitignore"), include_str!("../template/.gitignore"))
    .or_else(|_| Err(anyhow!("Failed to create .gitignore")))?;


  println!("Successfully initialized barley project");

  Ok(())
}