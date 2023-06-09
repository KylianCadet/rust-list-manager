use std::process::ExitCode;

use crate::cli::CliHandler;

use super::{r#error::ListManagerError, r#struct::ListManager, r#type::ReturnType};

impl CliHandler for ListManager {
    fn handle(&mut self, line: String) -> ReturnType {
        let mut tokens = line.splitn(2, ' ');
        let function_name: &str = tokens.next().unwrap_or_default();
        let arg: &str = tokens.next().unwrap_or_default();

        match function_name {
            "define" => self.define(arg),
            "display" => self.display(),
            "sum" => self.sum(),
            "sort" => self.sort(),
            "flatten" => self.flatten(),
            "chunks" => self.chunks(arg),
            "add" => self.add(arg),
            "cut" => self.cut(arg),
            "swap" => self.swap(arg),
            "help" => self.help(),
            "exit" => Ok(Some(ExitCode::SUCCESS)),
            _ => Err(Box::new(ListManagerError::InvalidFunction(
                function_name.to_string(),
            ))),
        }
    }
}
