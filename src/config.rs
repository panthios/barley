use serde::{Serialize, Deserialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize)]
pub struct Config {
  pub script: ScriptConfig,
  pub dependencies: HashMap<String, String>
}

#[derive(Serialize, Deserialize)]
pub struct ScriptConfig {
  pub name: String,
  pub version: String
}