use clap::Clap;

use rood::CausedResult;

mod pomodoro;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clap)]
#[clap(version = VERSION, author = "William Dussault")]
pub struct CLIRoot {
    /// Enable verbose output.
    #[clap(short = "v", long = "verbose", global = true)]
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
}

impl CLIRoot {
    pub fn run(&self) -> CausedResult<()> {
        let mut work_timer = pomodoro::WorkTimer::new(
            self.verbose,
            self.task_duration_minutes,
            self.short_break_duration_minutes,
            self.long_break_duration_minutes,
            self.task_count,
        );
        work_timer.run().unwrap();

        Ok(())
    }
}

fn main() {
    CLIRoot::parse().run().unwrap();
}
