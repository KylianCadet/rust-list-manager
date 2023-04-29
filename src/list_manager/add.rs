use super::{cerror::ListManagerError, ctype::ReturnType, list_manager::ListManager};

impl ListManager {
    pub fn add(&mut self, s: &str) -> ReturnType {
        if self.list.len() == 0 {
            return Err(Box::new(ListManagerError::NoAvailableListToPerformAction));
        }
        if s.len() == 0 {
            return Err(Box::new(ListManagerError::NoInput));
        }
        let add_value = s
            .trim()
            .parse::<i8>()
            .map_err(|_| ListManagerError::InvalidInput(s.to_string()))?;

        self.list
            .iter_mut()
            .for_each(|l| l.iter_mut().for_each(|v| *v += add_value));
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_with_no_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![];
        let err = list_manager.add("1").unwrap_err();
        assert_eq!(
            err.downcast_ref::<ListManagerError>(),
            Some(&ListManagerError::NoAvailableListToPerformAction)
        );
    }

    #[test]
    fn test_add_with_no_input() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3]];
        let err = list_manager.add("").unwrap_err();
        assert_eq!(
            err.downcast_ref::<ListManagerError>(),
            Some(&ListManagerError::NoInput)
        );
    }

    #[test]
    fn test_add_with_invalid_input() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3]];
        let err = list_manager.add("no").unwrap_err();
        assert_eq!(
            err.downcast_ref::<ListManagerError>(),
            Some(&ListManagerError::InvalidInput("no".to_string()))
        );
    }

    #[test]
    fn test_add_with_1_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3]];
        let expected_list = vec![vec![2, 3, 4]];
        list_manager.add("1").unwrap();
        assert_eq!(list_manager.list, expected_list);
    }

    #[test]
    fn test_add_with_2_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let expected_list = vec![vec![4, 5, 6], vec![7, 8, 9]];
        list_manager.add("3").unwrap();
        assert_eq!(list_manager.list, expected_list);
    }

    #[test]
    fn test_add_with_3_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12]];
        let expected_list = vec![
            vec![9, 10, 11, 12, 13],
            vec![14, 15, 16, 17, 18],
            vec![19, 20],
        ];
        list_manager.add("8").unwrap();
        assert_eq!(list_manager.list, expected_list);
    }

    #[test]
    fn test_add_with_negative_value() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12]];
        let expected_list = vec![vec![-1, 0, 1, 2, 3], vec![4, 5, 6, 7, 8], vec![9, 10]];
        list_manager.add("-2").unwrap();
        assert_eq!(list_manager.list, expected_list);
    }
}
