use std::fs;
use std::io::{self, Write};
use std::path::Path;

use chrono::Local;
use textwrap::fill;

pub fn write_entry(data_dir: &Path, entry_text: &str) -> io::Result<()> {
    let now = Local::now();
    let date_str = now.format("%Y-%m-%d").to_string();
    let time_str = now.format("%H:%M:%S").to_string();
    let wrapped_text = fill(entry_text, 80);
    let formatted_entry = format!("\n---\n**[{}]**\n{}\n", time_str, wrapped_text);

    let file_path = data_dir.join(format!("{}.md", date_str));
    let is_new_file = !file_path.exists();

    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&file_path)?;

    if is_new_file {
        let header = format!("# Journal for {}\n", date_str);
        file.write_all(header.as_bytes())?;
    }
    file.write_all(formatted_entry.as_bytes())?;

    Ok(())
}
