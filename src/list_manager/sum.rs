use super::{cerror::ListManagerError, ctype::ReturnType, list_manager::ListManager};

impl ListManager {
    pub fn sum(&mut self) -> ReturnType {
        match self.list.len() {
            x if x > 1 => Err(Box::new(ListManagerError::TooManyAvailableListToSum)),
            x if x == 0 => Err(Box::new(ListManagerError::NoAvailableListToPerformAction)),
            _ => {
                println!("{}", self.list.pop().unwrap().iter().sum::<i8>());
                Ok(None)
            }
        }
    }
}
