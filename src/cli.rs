use std::error::Error;
use std::io;
use std::process::ExitCode;

use crate::read_line::read_line;

pub trait CliHandler {
    fn handle(&mut self, line: String) -> Result<Option<ExitCode>, Box<dyn Error>>;
}

pub fn cli(mut cli_handler: impl CliHandler) -> Result<ExitCode, Box<dyn Error>> {
    let mut exit_code: Option<ExitCode> = None;

    while let None = exit_code {
        match read_line() {
            Ok(line) => {
                let handler_res = cli_handler.handle(line)?;
                exit_code = handler_res
            }
            Err(e) => match e.kind() {
                io::ErrorKind::UnexpectedEof => exit_code = Some(ExitCode::SUCCESS),
                _ => exit_code = Some(ExitCode::FAILURE),
            },
        }
    }

    exit_code.ok_or(Box::new(io::Error::new(
        io::ErrorKind::Other,
        "exit_code is None",
    )))
}
