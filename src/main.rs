use std::path::PathBuf;

use anyhow::Result;

use clap::Parser;

use pomodoro::{CLIFormat, Format};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[clap(version = VERSION, author = "William Dussault")]
pub struct CLIRoot {
    /// Enable verbose output.
    #[clap(short = 'v', long = "verbose", global = true)]
    verbose: bool,

    /// The duration of a task.
    #[clap(long = "task", default_value = "20")]
    task_duration_minutes: u64,

    /// The duration of a short break.
    #[clap(long = "short-break", default_value = "5")]
    short_break_duration_minutes: u64,

    /// The duration of a long break.
    #[clap(long = "long-break", default_value = "15")]
    long_break_duration_minutes: u64,

    /// The number of tasks between long breaks.
    #[clap(long = "task-count", default_value = "4")]
    task_count: u8,

    /// The format of the generated log.
    #[clap(long = "format", default_value = "None")]
    format: CLIFormat,
}

impl CLIRoot {
    pub fn run(&self) -> Result<()> {
        // Convert CLI Format to actual format.
        let fmt = match &self.format {
            CLIFormat::None => Format::None,
            CLIFormat::JSON => Format::JSON(PathBuf::from("./log.json")),
            CLIFormat::Markdown => Format::Markdown(PathBuf::from("./log.md")),
        };

        let mut work_timer = pomodoro::WorkTimer::new(
            self.task_duration_minutes,
            self.short_break_duration_minutes,
            self.long_break_duration_minutes,
            self.task_count,
            fmt,
        );

        work_timer.run()
    }
}

fn main() {
    xmt::init_default();

    if let Err(e) = CLIRoot::parse().run() {
        xmt::error!("{e}");
    }
}
