use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum ListManagerError {
    NoAvailableListToCut,
    TooManyAvailableListToSum,
    InvalidInput(String),
}

impl Error for ListManagerError {}

impl Display for ListManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListManagerError::NoAvailableListToCut => {
                f.write_str("No available list to cut, please define one")
            }
            ListManagerError::InvalidInput(s) => {
                f.write_fmt(format_args!("Invalid input \"{}\"", s))
            }
            ListManagerError::TooManyAvailableListToSum => {
                f.write_str("Too many available list to sum, must be used with only 1 list")
            }
        }
    }
}
