use std::thread;
use std::time::Duration;

use anyhow::Result;

use indicatif::{ProgressBar, ProgressStyle};

use rood::cli::OutputManager;
use rood::sys::notify;

use crate::{Format, WorkLog};

pub struct WorkTimer {
    output: OutputManager,
    task_length_minutes: u64,
    short_break_minutes: u64,
    long_break_minutes: u64,
    task_count: u8,

    log: WorkLog,
}

impl WorkTimer {
    pub fn new(
        verbose: bool,
        task_length_minutes: u64,
        short_break_minutes: u64,
        long_break_minutes: u64,
        task_count: u8,
        fmt: Format,
    ) -> WorkTimer {
        let log = WorkLog::new(fmt);

        WorkTimer {
            output: OutputManager::new(verbose),
            task_length_minutes,
            short_break_minutes,
            long_break_minutes,
            task_count,
            log,
        }
    }

    fn do_task(&self, task_name: &str) -> Result<()> {
        self.output.step(&format!("[Task] - {}", task_name));

        let task_seconds = self.task_length_minutes * 60;

        let pb = ProgressBar::new(task_seconds * 10);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.yellow} [{elapsed_precise}] [{bar:40.cyan/blue}] ({eta})")
                .progress_chars("#>-"),
        );

        for i in 0..(task_seconds * 10) {
            pb.set_position(i);
            thread::sleep(Duration::from_millis(100));
        }

        self.output.success("Done.");

        Ok(())
    }

    fn pause(&self, pause_time: Duration) -> Result<()> {
        let nb_of_seconds = pause_time.as_secs();

        self.output
            .step(&format!("[Break] - {} min", nb_of_seconds / 60));

        let pb = ProgressBar::new(nb_of_seconds * 10);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({eta})")
                .progress_chars("#>-"),
        );

        for i in 0..(nb_of_seconds * 10) {
            pb.set_position(i);
            thread::sleep(Duration::from_millis(100));
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        let mut task_count: u8 = 0;
        let mut last_task_name = String::new();

        loop {
            self.output.clear()?;
            let task_name = if last_task_name.is_empty() {
                self.output.prompt("Enter next task name: ")?
            } else {
                let ret_val = self
                    .output
                    .prompt(&format!("Enter next task name ({}): ", last_task_name))?;
                if ret_val.is_empty() {
                    last_task_name.clone()
                } else {
                    ret_val
                }
            };

            if task_name == "exit" {
                break;
            }

            self.output.clear()?;
            self.log.add(
                &task_name,
                Duration::from_secs(60 * self.task_length_minutes),
            )?;
            self.do_task(&task_name)?;
            last_task_name = task_name;

            task_count += 1;
            let pause_duration = Duration::from_secs(
                if task_count == self.task_count {
                    // Long break.
                    task_count = 0; // Reset task count.
                    notify::send(
                        "Long Break!",
                        "Time for a long break -- go and stretch your legs!",
                    )?;
                    self.long_break_minutes
                } else {
                    // Short break.
                    notify::send("Short Break!", "Time for a short break!")?;
                    self.short_break_minutes
                } * 60,
            );

            self.output.clear()?;
            self.pause(pause_duration)?;

            notify::send("Time's up!", "Time to enter a new task.")?;
        }

        Ok(())
    }
}
