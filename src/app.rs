use anyhow::{ensure, Context, Result};
use clap::Args;
use std::path::PathBuf;

use crate::{
  linkmanager::{self, Symlink},
  utils,
};

pub fn init(dotfile_path: PathBuf) -> Result<()> {
  let link = linkmanager::Symlink::new(dotfile_path.to_path_buf(), dirs::config_dir().unwrap().join("dotfiles"));
  ensure!(!link.is_linked(), "Already initialized");
  ensure!(!link.target.exists(), "Already initialized to different directory");
  ensure!(link.source.exists(), "Dotfile directory doesn't exist");
  link.link().context("Failed to initialize sidoma")
}

pub fn links_status() -> Result<()> {
  let dotfile_dir = utils::get_dot_file_dir().context("sidoma is not initialized")?;
  let lm = linkmanager::Linkmanager::new(dirs::config_dir().unwrap(), dotfile_dir)?;
  lm.status();
  Ok(())
}

pub fn links_create() -> Result<()> {
  let dotfile_dir = utils::get_dot_file_dir().context("sidoma is not initialized")?;
  let lm = linkmanager::Linkmanager::new(dirs::config_dir().unwrap(), dotfile_dir)?;
  lm.create()?;
  Ok(())
}

#[derive(Args)]
pub struct AddOptions {
  /// The directory or file to move
  pub path: PathBuf,
  /// Name to use in the dotfiles directory. Defaults to the name of the ... without any preceeding dots
  pub name: Option<String>,
}

pub fn _links_add(opts: AddOptions) -> Result<()> {
  let path = opts.path;
  ensure!(path.exists(), format!("{} doesn't exist", path.display()));

  // TODO remove . prefix if it exists
  ensure!(opts.name.is_none());
  let name = opts.name.unwrap_or(path.file_name().unwrap().to_str().unwrap().into());
  ensure!(!name.starts_with("."), "Name can't start with a dot");

  let dotfiles = utils::get_dot_file_dir()?;
  let target = dotfiles.join(name);
  // println!("{}", target.display());
  ensure!(!target.exists(), format!("{} already exists", target.display()));

  std::fs::rename(&path, &target)?;
  Symlink::new(target, path).link()?;

  // link file is not needed if path is a non-hidden dir inside .config and name is not specified

  // Move dir
  // create link file
  // create symlink

  Ok(())
}

#[cfg(test)]
mod tests {
  use std::{env, io::Write};

  use crate::{app::links_add, linkmanager::Symlink};
  use serial_test::serial;

  use super::*;
  use test_dir::{self, DirBuilder, FileType, TestDir};
  #[test]
  // any test that uses environment variables must be run serially,
  // as environment variables are shared with the currently running process.
  // consider using https://crates.io/crates/temp-env instead
  #[serial]
  fn create_links() {
    let fakehome = TestDir::temp()
      .create(".config", FileType::Dir)
      .create("dotfiles/bin/.link", FileType::EmptyFile)
      .create("dotfiles/bash/bashrc", FileType::EmptyFile)
      .create("dotfiles/bash/.link", FileType::EmptyFile)
      .create("dotfiles/nvim/init.vim", FileType::EmptyFile);

    env::set_var("HOME", fakehome.root());
    env::set_var("XDG_CONFIG_HOME", fakehome.path(".config"));
    std::os::unix::fs::symlink(fakehome.path("dotfiles"), fakehome.path(".config/dotfiles")).unwrap();

    let mut bash_linkfile = std::fs::File::create(fakehome.path("dotfiles/bash/.link")).unwrap();
    bash_linkfile.write_all(b"bashrc ~/.bashrc").unwrap();

    let mut bin_linkfile = std::fs::File::create(fakehome.path("dotfiles/bin/.link")).unwrap();
    bin_linkfile.write_all(b". ~/.bin").unwrap();

    println!("Using fake home: {}", fakehome.root().to_str().unwrap());
    links_status().unwrap();
    links_create().unwrap();
    links_status().unwrap();
    assert!(Symlink::new(fakehome.path("dotfiles/bin"), fakehome.path(".bin")).is_linked());
    assert!(Symlink::new(fakehome.path("dotfiles/bash/bashrc"), fakehome.path(".bashrc")).is_linked());
    assert!(Symlink::new(fakehome.path("dotfiles/nvim"), fakehome.path(".config/nvim")).is_linked());
  }
  #[test]
  #[serial]
  fn init() {
    let fakehome = TestDir::temp().create("dotfiles", FileType::Dir);

    env::set_var("HOME", fakehome.root());
    env::set_var("XDG_CONFIG_HOME", fakehome.path(".config"));
    std::env::set_current_dir(fakehome.root()).unwrap();

    super::init("dotfiles".into()).unwrap();
    assert!(Symlink::new(fakehome.path("dotfiles"), fakehome.path(".config/dotfiles")).is_linked());
  }
  #[test]
  #[serial]
  fn add() {
    let fakehome = TestDir::temp()
      .create(".config/nvim/init.vim", FileType::EmptyFile)
      .create("dotfiles", FileType::Dir);

    env::set_var("HOME", fakehome.root());
    env::set_var("XDG_CONFIG_HOME", fakehome.path(".config"));
    std::env::set_current_dir(fakehome.root()).unwrap();
    std::os::unix::fs::symlink(fakehome.path("dotfiles"), fakehome.path(".config/dotfiles")).unwrap();

    println!("Using fake home: {}", fakehome.root().to_str().unwrap());
    links_add(AddOptions {
      path: ".config/nvim".into(),
      name: None,
    })
    .unwrap();
    // std::thread::sleep(std::time::Duration::from_secs(20));

    // assert!(Symlink::new(fakehome.path("dotfiles/nvim"), fakehome.path(".config/nvim")).is_linked());
  }
}
