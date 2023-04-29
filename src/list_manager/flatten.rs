use super::{r#error::ListManagerError, r#struct::ListManager, r#type::ReturnType};

impl ListManager {
    pub fn flatten(&mut self) -> ReturnType {
        if self.list.len() < 2 {
            return Err(Box::new(ListManagerError::NotEnoughAvailableListToFlatten));
        }
        let new_list = self.list.concat();
        self.list.clear();
        self.list.push(new_list);
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_with_no_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![];
        let err = list_manager.flatten().unwrap_err();
        assert_eq!(
            err.downcast_ref::<ListManagerError>(),
            Some(&ListManagerError::NotEnoughAvailableListToFlatten)
        );
    }

    #[test]
    fn test_flatten_with_1_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![3, 2, 1]];
        let err = list_manager.flatten().unwrap_err();
        assert_eq!(
            err.downcast_ref::<ListManagerError>(),
            Some(&ListManagerError::NotEnoughAvailableListToFlatten)
        );
    }

    #[test]
    fn test_flatten_with_2_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let expected_list = vec![vec![1, 2, 3, 4, 5, 6]];
        list_manager.flatten().unwrap();
        assert_eq!(list_manager.list, expected_list);
    }

    #[test]
    fn test_flatten_with_3_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3], vec![4, 5, 6], vec![-1, -2, -3]];
        let expected_list = vec![vec![1, 2, 3, 4, 5, 6, -1, -2, -3]];
        list_manager.flatten().unwrap();
        assert_eq!(list_manager.list, expected_list);
    }
}
