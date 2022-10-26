mod symlink;
use anyhow::{ensure, Context, Result};
use colored::*;
use std::path::{Path, PathBuf};
pub use symlink::*;

use crate::utils::{expand_home_dir, get_subdirs, is_dotfile};

pub struct Linkmanager {
  default_target_path: PathBuf,
  dotfile_dir: PathBuf,
  links: Vec<Symlink>,
}

impl Linkmanager {
  pub fn new<T1, T2>(default_target_path: T1, dotfile_dir: T2) -> Result<Self>
  where
    T1: AsRef<Path>,
    T2: AsRef<Path>,
  {
    let default_target_path = expand_home_dir(&default_target_path);
    let dotfile_dir = expand_home_dir(&dotfile_dir);
    ensure!(dotfile_dir.is_absolute(), "Path to dotfiles must be absolute");
    let mut links = Vec::new();
    for dir in get_subdirs(&dotfile_dir)
      .context(format!("Cannot open dotfile dir {}", dotfile_dir.display()))?
      .into_iter()
      .filter(|dir| !is_dotfile(dir))
    {
      let link_path = dir.path().join(".link");
      if link_path.is_file() {
        links.extend(parse_link_file(&link_path)?.into_iter().map(|mut l| {
          l.source = dir.path().join(l.source);
          l
        }));
      } else {
        let target = default_target_path.join(dir.file_name());
        links.push(Symlink::new(dir.path(), target));
      }
    }
    Ok(Self {
      default_target_path,
      dotfile_dir,
      links,
    })
  }
  pub fn status(&self) {
    if self.links.is_empty() {
      return;
    }
    let link_strings = self
      .links
      .iter()
      .map(|l| {
        let s = display_path(l.source.strip_prefix(&self.dotfile_dir).unwrap_or(&l.source));
        let t = display_path(&l.target);
        match (l.is_linked(), l.target.exists(), l.source.exists()) {
          (true, _, _) => (s.normal(), t.normal(), "✔".green()),
          (_, true, _) => (s.normal(), t.bold().red(), "✘".red()),
          (_, _, false) => (s.strikethrough().red(), t.normal(), "✘".red()),
          _ => (s.normal(), t.normal(), "🞦".cyan()),
        }
      })
      .collect::<Vec<_>>();
    let max_src_len = link_strings.iter().max_by_key(|l| l.0.len()).unwrap().0.len();
    let max_tgt_len = link_strings.iter().max_by_key(|l| l.1.len()).unwrap().1.len();
    for sl in link_strings {
      println!(
        "{} {} {} {}",
        pad(sl.0, max_src_len),
        "🠄".dimmed(),
        pad(sl.1, max_tgt_len),
        sl.2
      );
    }
  }

  pub fn create(&self) -> Result<()> {
    for link in &self.links {
      if link.is_linked() {
        continue;
      }
      if link.target.exists() {
        let s = format!("Skipping link {link}, target exists").red();
        println!("{}", s);
        continue;
      }
      if !link.source.exists() {
        println!("Skipping link {link}, source doesn't exist");
        continue;
      }
      println!("{}", format!("Creating link {link}").green());
      link.link().context("Failed to create symlink")?;
    }
    Ok(())
  }
}

/// Pads a colored string to a specific size.
/// This function is necessary since standard rust fmt pad doesn't work well with ansi-strings.
fn pad(s: ColoredString, n: usize) -> String {
  if s.len() > n {
    s.to_string()
  } else {
    format!("{}{}", " ".repeat(n - s.len()), s)
  }
}
