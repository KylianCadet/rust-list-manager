use super::{cerror::ListManagerError, ctype::ReturnType, list_manager::ListManager};

impl ListManager {
    pub fn sum(&mut self) -> ReturnType {
        if self.list.len() > 1 {
            return Err(Box::new(ListManagerError::TooManyAvailableListToSum));
        }
        Ok(None)
    }
}
