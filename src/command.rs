use crate::config::Config;
use anyhow::{Result, anyhow};
use cargo_toml::{Manifest, Dependency, DependencyDetail};
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

  let barley_toml: String = include_str!("../template/barley.toml")
    .replace("{{ name }}", &current_dir.file_name().unwrap().to_string_lossy());

  fs::write(&current_dir.join("barley.toml"), barley_toml)
    .or_else(|_| Err(anyhow!("Failed to create barley.toml")))?;


  println!("Successfully initialized barley project");

  Ok(())
}

pub fn cmd_add(name: String) -> Result<()> {
  let current_dir = current_dir()
    .or_else(|_| Err(anyhow!("Failed to get current directory")))?;

  if !current_dir.join("barley.toml").exists() {
    return Err(anyhow!("barley.toml not found"));
  }

  let mut barley_toml = fs::read_to_string(&current_dir.join("barley.toml"))
    .or_else(|_| Err(anyhow!("Failed to read barley.toml")))?;

  let cargo_toml = fs::read_to_string(&current_dir.join("Cargo.toml"))
    .or_else(|_| Err(anyhow!("Failed to read Cargo.toml")))?;


  let mut config: Config = toml::from_str(&barley_toml)?;
  let mut cargo: Manifest = toml::from_str(&cargo_toml)?;

  if config.dependencies.contains_key(&name) {
    return Err(anyhow!("Module already exists"));
  }

  config.dependencies.insert(name.clone(), "latest".to_string());

  barley_toml = toml::to_string(&config)?;

  fs::write(&current_dir.join("barley.toml"), barley_toml)
    .or_else(|_| Err(anyhow!("Failed to write to barley.toml")))?;


  cargo.dependencies.insert(
    format!("blyx-{}", name),
    Dependency::Detailed(
      DependencyDetail {
        version: Some("*".to_string()),
        git: Some("https://github.com/panthios/barley-utils".to_string()),
        ..Default::default()
      }
    )
  );

  fs::write(&current_dir.join("Cargo.toml"), toml::to_string(&cargo)?)
    .or_else(|_| Err(anyhow!("Failed to write to Cargo.toml")))?;


  println!("Successfully added module {}", name);

  Ok(())
}