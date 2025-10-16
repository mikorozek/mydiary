use std::fs::create_dir_all;
use std::io::{self, Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::Command;

use chrono::Local;
use chrono_english::{Dialect, parse_date_string};
use clap::{Parser, Subcommand};
use directories::BaseDirs;

mod entry;

#[derive(Parser)]
#[command(name = "mydiary", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<CliCommand>,
}

#[derive(Subcommand)]
enum CliCommand {
    Add {
        #[arg(required = true)]
        text: Vec<String>,
    },
    View {
        #[arg(default_value = "today")]
        date: Vec<String>,
    },
}

fn open_specific_entry(data_dir: &Path, date_str: &str) -> io::Result<()> {
    let today = Local::now();
    let target_date = parse_date_string(date_str, today, Dialect::Uk).map_err(|e| {
        Error::new(
            ErrorKind::InvalidInput,
            format!("Invalid date format: {}", e),
        )
    })?;

    let file_path = data_dir.join(format!("{}.md", target_date.format("%Y-%m-%d")));

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nvim".to_string());

    let mut cmd = Command::new(editor).arg(&file_path).spawn()?;
    cmd.wait()?;
    Ok(())
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

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let data_dir = get_data_dir()?;
    match cli.command {
        Some(CliCommand::Add { text }) => {
            let entry_text = text.join(" ");
            entry::write_entry(&data_dir, &entry_text)
        }
        Some(CliCommand::View { date }) => {
            let date_str = date.join(" ");
            open_specific_entry(&data_dir, &date_str)
        }
        None => browse_entries(&data_dir),
    }
}
