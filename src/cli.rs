use std::path::PathBuf;

use crate::app::{init, links_create, links_status, AddOptions};
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  /// Manage symlinks
  Links {
    #[command(subcommand)]
    command: Links,
  },
  /// Initialize sidoma
  Init { dotfile_dir: PathBuf },
}

#[derive(Subcommand)]
enum Links {
  /// Show the status of all symlinks to be created
  /// "✔" Indicates that the link already exists.
  /// "🞦" Indicates that the link will be created.
  /// "✘" Indicates that the link cannot be created. This is happens if a file already exists at the target location or if the source doesn't exist
  Status,
  /// Create symlinks from every folder in the dotfile directory
  Create,
  /// Moves a directory/file to the dotfiles directory and creates a symlink to the old location
  Add(AddOptions),
}

pub fn run() -> Result<()> {
  let cli = Cli::parse();

  match cli.command {
    Commands::Links { command } => match command {
      Links::Status => links_status()?,
      Links::Create => links_create()?,
      _ => todo!(),
    },
    Commands::Init { dotfile_dir } => init(dotfile_dir)?,
  }
  Ok(())
}
