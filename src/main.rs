use std::{error::Error, process::ExitCode};

mod cli;
mod list_manager_handler;
mod read_line;

use cli::cli;
use list_manager_handler::ListManagerHandler;

fn main() -> Result<ExitCode, Box<dyn Error>> {
    cli(ListManagerHandler {})
}
