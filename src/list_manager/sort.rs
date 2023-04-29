use super::{ctype::ReturnType, list_manager::ListManager};

impl ListManager {
    pub fn sort(&mut self) -> ReturnType {
        self.list.iter_mut().for_each(|l| l.sort());
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_with_1_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![3, 2, 1]];
        let expected_list = vec![vec![1, 2, 3]];
        list_manager.sort().unwrap();
        assert_eq!(list_manager.list, expected_list);
    }

    #[test]
    fn test_sort_with_multiple_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![3, 2, 1], vec![5, 3, 2]];
        let expected_list = vec![vec![1, 2, 3], vec![2, 3, 5]];
        list_manager.sort().unwrap();
        assert_eq!(list_manager.list, expected_list);
    }

    #[test]
    fn test_sort_with_empty_list() {
        let mut list_manager = ListManager::new();
        let expected_list: Vec<Vec<i8>> = vec![];
        list_manager.sort().unwrap();
        assert_eq!(list_manager.list, expected_list);
    }

    #[test]
    fn test_sort_with_negative_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![-2, 10, -9]];
        let expected_list = vec![vec![-9, -2, 10]];
        list_manager.sort().unwrap();
        assert_eq!(list_manager.list, expected_list);
    }
}
