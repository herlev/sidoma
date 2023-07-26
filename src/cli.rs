use std::path::PathBuf;

use crate::{
  app::{init, links_create, links_status, AddOptions},
  templater,
};
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
  /// Manage template files
  Templates {
    #[command(subcommand)]
    command: Templates,
  },
  /// Initialize sidoma
  Init { dotfile_dir: PathBuf },
}

#[derive(Subcommand)]
enum Templates {
  /// Render a single template file
  Render {
    #[arg(short, long)]
    /// Toml file for specifying the template context
    context: Option<PathBuf>,
    /// Template file to be rendered
    file: PathBuf,
  },
  /// Render all .j2 files found in the dotfile directory
  RenderAll {
    #[arg(short, long)]
    /// Toml file for specifying the template context
    context: Option<PathBuf>,
  },
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
    Commands::Templates { command } => match command {
      Templates::Render { context, file } => templater::render_single_template(file, context)?,
      Templates::RenderAll { context } => templater::render_all(context)?,
    },
    Commands::Init { dotfile_dir } => init(dotfile_dir)?,
  }
  Ok(())
}
