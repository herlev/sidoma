use anyhow::{anyhow, Context, Result};
use colored::*;
use glob::glob;
use minijinja::Environment;
use std::path::PathBuf;
use toml::Table;

use crate::utils;

fn get_environment() -> Environment<'static> {
  let mut environment = Environment::new();
  environment.set_debug(true);
  environment.set_undefined_behavior(minijinja::UndefinedBehavior::Strict);
  environment
}

fn get_context(context_file: Option<PathBuf>) -> Result<Table> {
  match context_file {
    Some(path) => Ok(
      std::fs::read_to_string(path)
        .context("Failed reading context file")?
        .parse()
        .context("Failed parsing context file")?,
    ),
    None => Ok(Table::new()),
  }
}

pub fn render_single_template(template_file: PathBuf, context_file: Option<PathBuf>) -> Result<()> {
  let context = get_context(context_file)?;
  let content = std::fs::read_to_string(&template_file).context("Failed to read template file")?;
  let file_name = template_file.to_str().unwrap();
  render_template(file_name, &content, context).map_err(|e| anyhow!("{e:#}"))?;
  Ok(())
}

fn render_template(name: &str, content: &str, context: Table) -> Result<String, minijinja::Error> {
  let mut environment = get_environment();
  environment.add_template(name, &content)?;
  let s = environment.get_template(name).unwrap().render(context)?;
  Ok(s)
}

pub fn render_all(context_file: Option<PathBuf>) -> Result<()> {
  let dotfile_dir = utils::get_dot_file_dir().context("sidoma is not initialized")?;
  let prefix = format!("{}/", dotfile_dir.to_str().unwrap());
  let mut environment = get_environment();
  let context = get_context(context_file)?;
  let extension = ".j2";
  let glob = glob(dotfile_dir.join(format!("**/*{extension}")).to_str().unwrap()).expect("Failed to read glob pattern");
  for file in glob {
    let file = file.unwrap();
    let file_path = file.to_str().unwrap();
    let file_content = std::fs::read_to_string(&file)?;
    environment.set_debug(true);
    environment
      .add_template_owned(file_path.to_string(), file_content)
      .map_err(|e| anyhow!("{e:#}"))?;

    let file = file_path
      .strip_prefix(&prefix)
      .unwrap()
      .strip_suffix(extension)
      .unwrap();
    println!("{} {}", "rendering".green(), file);
    std::fs::write(
      file_path.strip_suffix(extension).unwrap(),
      environment
        .get_template(file_path)
        .unwrap()
        .render(&context)
        .map_err(|e| anyhow!("{e:#}"))?,
    )?;
  }

  Ok(())
}
