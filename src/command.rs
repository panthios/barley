use crate::utils;
use anyhow::{Result, anyhow};
use cargo_toml::{Dependency, DependencyDetail};



pub fn cmd_init(lib: bool) -> Result<()> {
  if !utils::is_empty(None)? {
    return Err(anyhow!("Directory is not empty"));
  }

  utils::create_dir(None, "src")?;

  if lib {

    utils::write_file(None, "src/lib.rs", include_str!("../template/library/lib.rs"))?;

    let cargo_toml: String = include_str!("../template/library/Cargo.toml")
      .replace("{{ name }}", &format!("blyx-{}", &utils::barley_name(None)?));

    utils::write_file(None, "Cargo.toml", &cargo_toml)?;
    utils::write_file(None, ".gitignore", include_str!("../template/library/.gitignore"))?;

    let barley_toml: String = include_str!("../template/library/barley.toml")
      .replace("{{ name }}", &utils::barley_name(None)?);

    utils::write_file(None, "barley.toml", &barley_toml)?;

  } else {

    utils::write_file(None, "src/main.rs", include_str!("../template/script/main.rs"))?;

    let cargo_toml: String = include_str!("../template/script/Cargo.toml")
      .replace("{{ name }}", &format!("blyscript-{}", &utils::barley_name(None)?));

    utils::write_file(None, "Cargo.toml", &cargo_toml)?;
    utils::write_file(None, ".gitignore", include_str!("../template/script/.gitignore"))?;

    let barley_toml: String = include_str!("../template/script/barley.toml")
      .replace("{{ name }}", &utils::barley_name(None)?);

    utils::write_file(None, "barley.toml", &barley_toml)?;

  }

  println!("Successfully initialized barley project");

  Ok(())
}

pub fn cmd_add(name: String) -> Result<()> {
  if !utils::is_barley(None)? {
    return Err(anyhow!("Not a barley project"));
  }

  if !utils::is_crate(None)? {
    return Err(anyhow!("Not a crate"));
  }

  let mut config = utils::get_barley(None)?;
  let mut cargo = utils::get_cargo(None)?;

  if let Some(_) = config.library {
    return Err(anyhow!("Project is a library"));
  }

  if config.dependencies.contains_key(&name) {
    return Err(anyhow!("Module already exists"));
  }

  config.dependencies.insert(name.clone(), "latest".to_string());
  utils::set_barley(None, config)?;

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

  utils::set_cargo(None, cargo)?;

  println!("Successfully added module {}", name);

  Ok(())
}