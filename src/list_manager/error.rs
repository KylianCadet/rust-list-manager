use std::error::Error;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum ListManagerError {
    NoAvailableListToPerformAction,
    TooManyAvailableListToSum,
    NotEnoughAvailableListToFlatten,
    InvalidInput(String),
    InvalidFunction(String),
    NoInput,
    IndexOutOfRange(Vec<i8>),
}

impl Error for ListManagerError {}

impl Display for ListManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListManagerError::InvalidInput(s) => {
                f.write_fmt(format_args!("Invalid input \"{}\"", s))
            }
            ListManagerError::TooManyAvailableListToSum => {
                f.write_str("Too many available list to sum, must be used with only 1 list")
            }
            ListManagerError::InvalidFunction(s) => f.write_fmt(format_args!(
                "Invalid function \"{}\". Type \"help\" for command list",
                s
            )),
            ListManagerError::NoAvailableListToPerformAction => {
                f.write_str("No available list to perform action, please define one")
            }
            ListManagerError::NotEnoughAvailableListToFlatten => {
                f.write_str("Not enough available list to flatten, please define at least 2")
            }
            ListManagerError::NoInput => f.write_str("Function expected an input"),
            ListManagerError::IndexOutOfRange(l) => {
                f.write_fmt(format_args!("Index out of range for list {:?}", l))
            }
        }
    }
}
