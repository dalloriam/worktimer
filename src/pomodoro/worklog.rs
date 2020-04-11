use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::Result;

use chrono::{DateTime, Local};

use serde::Serialize;

pub enum Format {
    JSON(PathBuf),
    Markdown(PathBuf),
    None,
}

#[derive(Serialize)]
struct SerializationEntry {
    pub duration: u64, // Minutes
    pub time: String,  // Formatted
    pub name: String,
}

struct Entry {
    duration: Duration,
    name: String,
    time: DateTime<Local>,
}

impl Entry {
    pub fn new(task: &str, duration: Duration) -> Entry {
        Entry {
            name: String::from(task),
            duration,
            time: Local::now(),
        }
    }

    fn to_entry(&self) -> SerializationEntry {
        SerializationEntry {
            name: self.name.clone(),
            duration: self.duration.as_secs() / 60,
            time: format!("{}", self.time),
        }
    }

    fn write_json(&self, path: &Path) -> Result<()> {
        let serialized = serde_json::to_string(&self.to_entry())? + "\n";

        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .open(path)?;

        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    fn write_markdown(&self, path: &Path) -> Result<()> {
        if !path.exists() {
            let line = format!("# Log for {}\n\n", self.time.format("%A, %h %d %Y"));
            fs::write(path, line)?;
        }

        let serialized = format!(
            "* {} - {} ({} min)\n",
            self.time.format("%H:%M"),
            self.name,
            self.duration.as_secs() / 60
        );

        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .open(path)?;

        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn write(&self, fmt: &Format) -> Result<()> {
        match fmt {
            Format::None => Ok(()),
            Format::JSON(path) => self.write_json(path),
            Format::Markdown(path) => self.write_markdown(path),
        }
    }
}

pub struct WorkLog {
    fmt: Format,
}

impl WorkLog {
    pub fn new(fmt: Format) -> WorkLog {
        WorkLog { fmt }
    }

    pub fn add(&mut self, task: &str, duration: Duration) -> Result<()> {
        let entry = Entry::new(task, duration);
        entry.write(&self.fmt)
    }
}

impl Default for WorkLog {
    fn default() -> WorkLog {
        WorkLog::new(Format::None) // Default to no-op logger
    }
}
