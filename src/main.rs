// #![allow(unused_imports)]
// #![allow(dead_code)]
mod app;
mod cli;
mod linkmanager;
mod templater;
mod utils;

use anyhow::Result;

// TODO: find a way to link desktop files, maybe expand globs?
// *.desktop ~/.local/share/applications/

fn main() -> Result<()> {
  cli::run()?;
  Ok(())
}
