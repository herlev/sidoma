use std::{
  fs,
  path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use path_clean::PathClean;

pub fn get_dot_file_dir() -> Result<PathBuf> {
  let config_dir = dirs::config_dir().unwrap();
  let p = config_dir.join("dotfiles");
  fs::canonicalize(&p).context(format!("Path {p:?} does not exist"))
}

pub fn expand_home_dir<P: AsRef<Path>>(path: P) -> PathBuf {
  let path = path.as_ref();
  if !path.starts_with("~") {
    return path.into();
  }
  Path::new(&std::env::var("HOME").expect("$HOME environment variable not set"))
    .join(path.strip_prefix("~").unwrap_or(path))
}

pub fn absolute_path(path: impl AsRef<Path>) -> std::io::Result<PathBuf> {
  let path = path.as_ref();

  let absolute_path = if path.is_absolute() {
    path.to_path_buf()
  } else {
    std::env::current_dir()?.join(path)
  }
  .clean();

  Ok(absolute_path)
}

pub fn is_dotfile(entry: &fs::DirEntry) -> bool {
  entry.file_name().to_str().unwrap().starts_with('.')
}

/// Returns a list of all the subdirectories of a given directory
pub fn get_subdirs<P: AsRef<Path>>(path: P) -> Result<Vec<fs::DirEntry>> {
  Ok(
    fs::read_dir(path)?
      .collect::<Result<Vec<_>, _>>()?
      .into_iter()
      .filter(|entry| entry.path().is_dir())
      .collect(),
  )
}
