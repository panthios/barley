use crate::schema;
use anyhow::{Result, anyhow};
use cargo_toml::Manifest;
use std::{
  env::current_dir,
  path::{PathBuf, Path},
  fs
};



pub fn barley_name(path: Option<PathBuf>) -> Result<String> {
  let dir = current_dir()
    .or_else(|_| Err(anyhow!("Failed to get current directory")))?;

  let dir = match path {
    Some(path) => dir.join(path),
    None => dir
  };

  let name = dir.file_name()
    .ok_or_else(|| anyhow!("Failed to get directory name"))?
    .to_string_lossy()
    .to_string();

  Ok(name)
}

pub fn is_empty(path: Option<PathBuf>) -> Result<bool> {
  let dir = current_dir()
    .or_else(|_| Err(anyhow!("Failed to get current directory")))?;

  let dir = match path {
    Some(path) => dir.join(path),
    None => dir
  };

  let is_empty = {
    let mut entries = fs::read_dir(&dir)
      .or_else(|_| Err(anyhow!("Failed to read current directory")))?;

    entries.next().is_none()
  };

  Ok(is_empty)
}

pub fn is_crate(path: Option<PathBuf>) -> Result<bool> {
  let dir = current_dir()
    .or_else(|_| Err(anyhow!("Failed to get current directory")))?;

  let dir = match path {
    Some(path) => dir.join(path),
    None => dir
  };

  let is_crate = dir.join("Cargo.toml").exists();

  Ok(is_crate)
}

pub fn is_barley(path: Option<PathBuf>) -> Result<bool> {
  let dir = current_dir()
    .or_else(|_| Err(anyhow!("Failed to get current directory")))?;

  let dir = match path {
    Some(path) => dir.join(path),
    None => dir
  };

  let is_barley = dir.join("barley.toml").exists();

  Ok(is_barley)
}

pub fn get_barley(path: Option<PathBuf>) -> Result<schema::Config> {
  let dir = current_dir()
    .or_else(|_| Err(anyhow!("Failed to get current directory")))?;

  let dir = match path {
    Some(path) => dir.join(path),
    None => dir
  };

  let barley_toml = fs::read_to_string(dir.join("barley.toml"))
    .or_else(|_| Err(anyhow!("Failed to read barley.toml")))?;

  let config: schema::Config = toml::from_str(&barley_toml)
    .or_else(|_| Err(anyhow!("Failed to parse barley.toml")))?;

  Ok(config)
}

pub fn set_barley(path: Option<PathBuf>, config: schema::Config) -> Result<()> {
  let dir = current_dir()
    .or_else(|_| Err(anyhow!("Failed to get current directory")))?;

  let dir = match path {
    Some(path) => dir.join(path),
    None => dir
  };

  let barley_toml = toml::to_string(&config)
    .or_else(|_| Err(anyhow!("Failed to serialize barley.toml")))?;

  fs::write(dir.join("barley.toml"), barley_toml)
    .or_else(|_| Err(anyhow!("Failed to write barley.toml")))?;

  Ok(())
}

pub fn get_cargo(path: Option<PathBuf>) -> Result<Manifest> {
  let dir = current_dir()
    .or_else(|_| Err(anyhow!("Failed to get current directory")))?;

  let dir = match path {
    Some(path) => dir.join(path),
    None => dir
  };

  let cargo_toml = fs::read_to_string(dir.join("Cargo.toml"))
    .or_else(|_| Err(anyhow!("Failed to read Cargo.toml")))?;

  let cargo: Manifest = toml::from_str(&cargo_toml)
    .or_else(|_| Err(anyhow!("Failed to parse Cargo.toml")))?;

  Ok(cargo)
}

pub fn set_cargo(path: Option<PathBuf>, cargo: Manifest) -> Result<()> {
  let dir = current_dir()
    .or_else(|_| Err(anyhow!("Failed to get current directory")))?;

  let dir = match path {
    Some(path) => dir.join(path),
    None => dir
  };

  let cargo_toml = toml::to_string(&cargo)
    .or_else(|_| Err(anyhow!("Failed to serialize Cargo.toml")))?;

  fs::write(dir.join("Cargo.toml"), cargo_toml)
    .or_else(|_| Err(anyhow!("Failed to write Cargo.toml")))?;

  Ok(())
}

pub fn write_file<P: AsRef<Path>>(base: Option<P>, path: P, contents: &str) -> Result<()> {
  let dir = match base {
    Some(base) => current_dir()?.join(base),
    None => current_dir()?
  };

  let path = dir.join(path);

  fs::write(&path, contents)
    .or_else(|_| Err(anyhow!(format!("Failed to write file: {}", path.to_string_lossy()))))?;

  Ok(())
}

pub fn create_dir<P: AsRef<Path>>(base: Option<PathBuf>, path: P) -> Result<()> {
  let dir = match base {
    Some(base) => current_dir()?.join(base),
    None => current_dir()?
  };

  let path = dir.join(path);

  fs::create_dir_all(&path)
    .or_else(|_| Err(anyhow!(format!("Failed to create directory: {}", path.to_string_lossy()))))?;

  Ok(())
}