use std::{error::Error, process::ExitCode};

pub type ReturnType = Result<Option<ExitCode>, Box<dyn Error>>;
