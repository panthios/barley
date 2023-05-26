use crate::schema;
use anyhow::{Result, anyhow};
use cargo_toml::Manifest;
use git2::Repository;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
  env::current_dir,
  path::{PathBuf, Path},
  fs
};

lazy_static! {
  pub static ref RE_BARLEY_NAME: Regex = Regex::new(r#"^[a-zA-Z][a-zA-Z0-9_-]*$"#).unwrap();
}


pub struct Context {
  pub path: PathBuf
}

impl Context {
  pub fn new(path: Option<PathBuf>) -> Result<Self> {
    let curdir = current_dir()
      .or_else(|_| Err(anyhow!("Failed to get current directory")))?;

    let path = match path {
      Some(path) => curdir.join(path),
      None => curdir
    };

    if !path.exists() {
      fs::create_dir_all(&path)
        .or_else(|_| Err(anyhow!("Failed to create directory")))?;
    }

    Ok(Self { path })
  }

  pub fn barley_name(&self) -> Result<String> {
    let name = self.path.file_name()
      .ok_or_else(|| anyhow!("Failed to get directory name"))?
      .to_string_lossy()
      .to_string();

    Ok(name)
  }

  pub fn is_empty(&self) -> Result<bool> {
    let is_empty = {
      let mut entries = fs::read_dir(&self.path)
        .or_else(|_| Err(anyhow!("Failed to read current directory")))?;

      entries.next().is_none()
    };

    Ok(is_empty)
  }

  pub fn is_crate(&self) -> Result<bool> {
    let is_crate = self.path.join("Cargo.toml").exists();

    Ok(is_crate)
  }

  pub fn is_barley(&self) -> Result<bool> {
    let is_barley = self.path.join("barley.toml").exists();

    Ok(is_barley)
  }

  pub fn is_barley_script(&self) -> Result<bool> {
    if !self.is_barley()? {
      return Ok(false);
    }

    let config = self.barley_config()?;

    Ok(config.script.is_some())
  }

  pub fn is_in_repository(&self) -> Result<bool> {
    let repo = Repository::discover(&self.path)
      .or_else(|_| Err(anyhow!("Failed to discover repository")))?;

    let is_in_repository = repo.is_bare() || repo.workdir().is_some();

    Ok(is_in_repository)
  }

  pub fn barley_config(&self) -> Result<schema::Config> {
    let barley_toml = fs::read_to_string(self.path.join("barley.toml"))
      .or_else(|_| Err(anyhow!("Failed to read barley.toml")))?;

    let config: schema::Config = toml::from_str(&barley_toml)
      .or_else(|_| Err(anyhow!("Failed to parse barley.toml")))?;

    Ok(config)
  }

  pub fn barley_lockfile(&self) -> Result<schema::Lockfile> {
    if !self.path.join("barley.lock").exists() {
      return Ok(Default::default());
    }

    let lockfile_toml = fs::read_to_string(self.path.join("barley.lock"))
      .or_else(|_| Err(anyhow!("Failed to read barley.lock")))?;

    let lockfile: schema::Lockfile = toml::from_str(&lockfile_toml)
      .or_else(|_| Err(anyhow!("Failed to parse barley.lock")))?;

    Ok(lockfile)
  }

  pub fn cargo_config(&self) -> Result<Manifest> {
    let cargo_toml = fs::read_to_string(self.path.join("Cargo.toml"))
      .or_else(|_| Err(anyhow!("Failed to read Cargo.toml")))?;

    let cargo: Manifest = toml::from_str(&cargo_toml)
      .or_else(|_| Err(anyhow!("Failed to parse Cargo.toml")))?;

    Ok(cargo)
  }

  pub fn set_barley_config(&self, config: schema::Config) -> Result<&Self> {
    let barley_toml = toml::to_string(&config)
      .or_else(|_| Err(anyhow!("Failed to serialize barley.toml")))?;

    fs::write(self.path.join("barley.toml"), barley_toml)
      .or_else(|_| Err(anyhow!("Failed to write barley.toml")))?;

    Ok(self)
  }

  pub fn set_barley_lockfile(&self, lockfile: schema::Lockfile) -> Result<&Self> {
    let lockfile_toml = toml::to_string(&lockfile)
      .or_else(|_| Err(anyhow!("Failed to serialize barley.lock")))?;

    fs::write(self.path.join("barley.lock"), lockfile_toml)
      .or_else(|_| Err(anyhow!("Failed to write barley.lock")))?;

    Ok(self)
  }

  pub fn set_cargo_config(&self, cargo: Manifest) -> Result<&Self> {
    let cargo_toml = toml::to_string(&cargo)
      .or_else(|_| Err(anyhow!("Failed to serialize Cargo.toml")))?;

    fs::write(self.path.join("Cargo.toml"), cargo_toml)
      .or_else(|_| Err(anyhow!("Failed to write Cargo.toml")))?;

    Ok(self)
  }

  pub fn create_dir<P: AsRef<Path>>(&self, name: P) -> Result<&Self> {
    fs::create_dir_all(self.path.join(name))
      .or_else(|_| Err(anyhow!("Failed to create directory")))?;

    Ok(self)
  }

  pub fn write_file<P: AsRef<Path>>(&self, name: P, content: &str) -> Result<&Self> {
    fs::write(self.path.join(name), content)
      .or_else(|_| Err(anyhow!("Failed to write file")))?;

    Ok(self)
  }

  pub fn run_cargo(&self, args: &[&str]) -> Result<()> {
    let mut command = std::process::Command::new("cargo");
    
    let status = command
      .args(args)
      .current_dir(&self.path)
      .status()?;

    if !status.success() {
      return Err(anyhow!("Failed to run cargo"));
    }

    Ok(())
  }
}