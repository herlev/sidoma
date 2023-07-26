use anyhow::{ensure, Context, Result};
use itertools::*;
use path_clean::PathClean;
use std::path::{Path, PathBuf};
use std::{fmt, fs};

use crate::utils::{self, expand_home_dir};

pub fn display_path(path: &Path) -> String {
  let home = std::env::var("HOME").unwrap();
  if path.starts_with(&home) {
    return Path::new("~")
      .join(path.strip_prefix(home).unwrap())
      .display()
      .to_string();
  }
  path.as_os_str().to_str().unwrap().into()
}

#[derive(Debug)]
pub struct Symlink {
  pub source: PathBuf,
  pub target: PathBuf,
}

impl Symlink {
  /// Creates a new symlink object.
  pub fn new(source: PathBuf, target: PathBuf) -> Self {
    Self { source, target }
  }
  /// Parses a single line of a .link file, which has the following format:
  /// ```
  /// source1 target1
  /// source2 target2
  /// ...
  /// ```
  /// The source is relative to the directory of the .link file, while the target must be an absolute path.
  pub fn parse(s: &str) -> Result<Self> {
    let (source, target) = s
      .split(' ')
      .map(|path| Path::new(path).to_path_buf())
      .collect_tuple()
      .context("Link specifications must be of the form: <source> <target>\nPaths cannot contain spaces")?;
    let target = expand_home_dir(&target);
    ensure!(
      source.is_relative(),
      "Source \"{}\" must be a relative path",
      source.to_str().unwrap()
    );
    ensure!(
      !source.clean().starts_with(".."),
      "Source \"{}\" cannot reference parent directories",
      source.to_str().unwrap()
    );
    ensure!(
      target.is_absolute(),
      "Target \"{}\" must be an absolute path",
      target.to_str().unwrap()
    );
    Ok(Self::new(source, target))
  }
  /// Returns true if the target is a symlink and points to a file or directory at source.
  pub fn is_linked(&self) -> bool {
    if self.source.exists() && self.target.is_symlink() {
      let target = fs::canonicalize(&self.target).expect("Failed to canonicalize target path");
      if utils::absolute_path(&self.source).unwrap() == utils::absolute_path(target).unwrap() {
        return true;
      }
    }
    false
  }

  /// Creates a relative symlink and all parent directories of the target.
  pub fn link(&self) -> Result<()> {
    let source = utils::absolute_path(&self.source)?;
    let target = utils::absolute_path(&self.target)?;
    fs::create_dir_all(
      target
        .parent()
        .unwrap_or_else(|| panic!("Failed to create parent directory of {}", self.target.display())),
    )?;
    let relative_source = pathdiff::diff_paths(&source, &target.parent().unwrap()).unwrap();
    std::os::unix::fs::symlink(relative_source, &target)?;
    Ok(())
  }

  /// Removes the symlink.
  pub fn _unlink(&self) -> Result<()> {
    todo!()
  }
}

impl fmt::Display for Symlink {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} 🠆 {}", display_path(&self.source), display_path(&self.target))
  }
}

/// Parses a .link file and returns a list of Symlink
pub fn parse_link_file(path: &Path) -> Result<Vec<Symlink>> {
  fs::read_to_string(path)
    .unwrap()
    .lines()
    .map(|line| line.trim())
    .enumerate()
    .filter(|(_, line)| !line.is_empty() && !line.starts_with('#'))
    .map(|(line_num, line)| {
      Symlink::parse(line).context(format!(
        "Failed to parse line {} of {}",
        line_num + 1,
        display_path(path),
      ))
    })
    .collect::<Result<Vec<_>>>()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_expand_home_dir() {
    let home = std::env::var("HOME").unwrap();
    assert_eq!(Path::new(&format!("{home}/test")), expand_home_dir("~/test"));
    assert_eq!(Path::new("test/~/test"), expand_home_dir("test/~/test"));
  }
  #[test]
  fn test_symlink() {
    assert!(Symlink::parse(". ~/.bin").is_ok());
    assert!(Symlink::parse("test test").is_err());
    assert!(Symlink::parse("/test ~/test").is_err());
  }
}
