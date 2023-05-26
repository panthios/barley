use serde::{Serialize, Deserialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize)]
pub struct Config {
  pub script: Option<ScriptConfig>,
  pub library: Option<LibraryConfig>,
  pub dependencies: HashMap<String, String>
}

#[derive(Serialize, Deserialize)]
pub struct ScriptConfig {
  pub name: String,
  pub version: String
}

#[derive(Serialize, Deserialize)]
pub struct LibraryConfig {
  pub name: String,
  pub version: String
}


#[derive(Serialize, Deserialize, Default)]
pub struct Lockfile {
  pub dependencies: HashMap<String, LockedDependency>
}

#[derive(Serialize, Deserialize)]
pub struct LockedDependency {
  pub version: String,
  pub cargo_name: String,
  pub cargo_version: String
}