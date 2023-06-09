use std::{error::Error, process::ExitCode};

mod cli;
mod list_manager;
mod read_line;

use cli::cli;
use list_manager::r#struct::ListManager;

fn main() -> Result<ExitCode, Box<dyn Error>> {
    cli(ListManager::new())
}
