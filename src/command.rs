use crate::{utils, schema::LockedDependency};
use anyhow::{Result, anyhow};
use cargo_toml::{Dependency, DependencyDetail};



pub fn cmd_init(ctx: utils::Context, lib: bool) -> Result<()> {
  if !ctx.is_empty()? {
    return Err(anyhow!("Directory is not empty"));
  }

  if lib {

    let cargo_toml: String = include_str!("../template/library/Cargo.toml")
      .replace("{{ name }}", &format!("blyx-{}", &ctx.barley_name()?));

    let barley_toml: String = include_str!("../template/library/barley.toml")
      .replace("{{ name }}", &ctx.barley_name()?);

    ctx.create_dir("src")?
      .write_file("src/lib.rs", include_str!("../template/library/lib.rs"))?
      .write_file("Cargo.toml", &cargo_toml)?
      .write_file("barley.toml", &barley_toml)?
      .write_file(".gitignore", include_str!("../template/library/.gitignore"))?;

  } else {

    let cargo_toml: String = include_str!("../template/script/Cargo.toml")
      .replace("{{ name }}", &format!("blyscript-{}", &ctx.barley_name()?));

    let barley_toml: String = include_str!("../template/script/barley.toml")
      .replace("{{ name }}", &ctx.barley_name()?);

    ctx.create_dir("src")?
      .write_file("src/main.rs", include_str!("../template/script/main.rs"))?
      .write_file("Cargo.toml", &cargo_toml)?
      .write_file("barley.toml", &barley_toml)?
      .write_file(".gitignore", include_str!("../template/script/.gitignore"))?;
  }

  println!("Successfully initialized barley project");

  Ok(())
}

pub fn cmd_add(ctx: utils::Context, name: String) -> Result<()> {
  if ctx.is_empty()? {
    return Err(anyhow!("Directory is empty"));
  }

  if !ctx.is_barley_script()? {
    return Err(anyhow!("Directory is not a barley script"));
  }

  let mut config = ctx.barley_config()?;
  let mut cargo = ctx.cargo_config()?;
  let mut lockfile = ctx.barley_lockfile()?;

  if config.dependencies.contains_key(&name) {
    return Err(anyhow!("Module already exists"));
  }

  config.dependencies.insert(name.clone(), "0.0.1".to_string());

  cargo.dependencies.insert(
    format!("blyx-{}", name),
    Dependency::Detailed(
      DependencyDetail {
        version: Some("0.0.1".to_string()),
        git: Some("https://github.com/panthios/barley-utils".to_string()),
        ..Default::default()
      }
    )
  );

  lockfile.dependencies.insert(
    name.clone(),
    LockedDependency {
      version: "0.0.1".to_string(),
      cargo_version: "0.0.1".to_string(),
      cargo_name: format!("blyx-{}", name)
    }
  );

  ctx.set_barley_config(config)?
    .set_cargo_config(cargo)?
    .set_barley_lockfile(lockfile)?;

  println!("Successfully added module {}", name);

  Ok(())
}