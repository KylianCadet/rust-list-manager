use std::process::ExitCode;

use crate::cli::CliHandler;

use super::{ctype::ReturnType, list_manager::ListManager};

impl CliHandler for ListManager {
    fn handle(&mut self, line: String) -> ReturnType {
        let mut tokens = line.splitn(2, ' ');
        let function_name: &str = tokens.next().unwrap_or_default();
        let arg: &str = tokens.next().unwrap_or_default();

        match function_name {
            "define" => self.define(arg),
            "display" => self.display(),
            "exit" => Ok(Some(ExitCode::SUCCESS)),
            _ => self.unknown(function_name),
        }
    }
}
