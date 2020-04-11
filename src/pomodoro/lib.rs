mod worklog;
mod worktimer;

pub use worklog::Format;
use worklog::WorkLog;
pub use worktimer::WorkTimer;

use std::convert::Infallible;
use std::str::FromStr;

pub enum CLIFormat {
    JSON,
    Markdown,
    None,
}

impl FromStr for CLIFormat {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<CLIFormat, Self::Err> {
        Ok(match s {
            "json" => CLIFormat::JSON,
            "markdown" => CLIFormat::Markdown,
            _ => CLIFormat::None,
        })
    }
}
