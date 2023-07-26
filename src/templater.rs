use anyhow::{anyhow, Context, Result};
use minijinja::Environment;
use std::path::PathBuf;
use toml::Table;

pub fn render_single_template(template_file: PathBuf, context_file: Option<PathBuf>) -> Result<()> {
  let context = match context_file {
    Some(path) => std::fs::read_to_string(path)
      .context("Failed reading context file")?
      .parse()
      .context("Failed parsing context file")?,
    None => Table::new(),
  };
  let content = std::fs::read_to_string(&template_file).context("Failed to read template file")?;
  let file_name = template_file.to_str().unwrap();
  match render_template(file_name, &content, context) {
    Ok(s) => println!("{}", s),
    Err(e) => return Err(anyhow!("{e:#}")),
  }
  Ok(())
}

fn render_template(name: &str, content: &str, context: Table) -> Result<String, minijinja::Error> {
  let mut environment = Environment::new();
  environment.set_debug(true);
  environment.add_template(name, &content)?;
  environment.set_undefined_behavior(minijinja::UndefinedBehavior::Strict);
  let s = environment.get_template(name).unwrap().render(context)?;
  Ok(s)
}
