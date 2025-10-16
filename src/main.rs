use std::fs::create_dir_all;
use std::io::{self, Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::Command;

use clap::Parser;
use directories::BaseDirs;

mod entry;

#[derive(Parser)]
#[command(name = "mydiary", version, about)]
struct Cli {
    entry: Option<String>,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let data_dir = get_data_dir()?;

    match cli.entry {
        Some(entry_text) => entry::write_entry(&data_dir, &entry_text),
        None => browse_entries(&data_dir),
    }
}

fn get_data_dir() -> io::Result<PathBuf> {
    let base_dirs = BaseDirs::new()
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Could not find home directory."))?;
    let data_dir = base_dirs.data_dir().join("mydiary");
    create_dir_all(&data_dir)?;
    Ok(data_dir)
}

fn browse_entries(data_dir: &Path) -> io::Result<()> {
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nvim".to_string());

    let mut cmd = Command::new(editor).arg(data_dir).spawn()?;

    cmd.wait()?;
    Ok(())
}
