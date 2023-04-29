use super::{cerror::ListManagerError, ctype::ReturnType, list_manager::ListManager};

impl ListManager {
    fn _swap(slice: &mut Vec<i8>, index: usize) {
        let mut tmp = slice[index..].to_vec();
        slice.truncate(index);
        tmp.reverse();
        slice.extend_from_slice(&tmp);
        slice.reverse();
    }

    pub fn swap(&mut self, s: &str) -> ReturnType {
        if self.list.len() == 0 {
            return Err(Box::new(ListManagerError::NoAvailableListToPerformAction));
        }
        if s.len() == 0 {
            return Err(Box::new(ListManagerError::NoInput));
        }
        let swap_index_value = s
            .trim()
            .parse::<usize>() // u8 because input cannot be negative
            .map_err(|_| ListManagerError::InvalidInput(s.to_string()))?;

        for l in self.list.iter() {
            if swap_index_value > l.len() - 1 {
                return Err(Box::new(ListManagerError::IndexOutOfRange(l.to_vec())));
            }
        }

        self.list
            .iter_mut()
            .for_each(|l| Self::_swap(l, swap_index_value + 1));

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cut_with_no_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![];
        let err = list_manager.swap("1").unwrap_err();
        assert_eq!(
            err.downcast_ref::<ListManagerError>(),
            Some(&ListManagerError::NoAvailableListToPerformAction)
        );
    }

    #[test]
    fn test_cut_with_no_input() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3]];
        let err = list_manager.swap("").unwrap_err();
        assert_eq!(
            err.downcast_ref::<ListManagerError>(),
            Some(&ListManagerError::NoInput)
        );
    }

    #[test]
    fn test_cut_with_invalid_input() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3]];
        let err = list_manager.swap("no").unwrap_err();
        assert_eq!(
            err.downcast_ref::<ListManagerError>(),
            Some(&ListManagerError::InvalidInput("no".to_string()))
        );
    }

    #[test]
    fn test_cut_with_out_of_index_input() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3]];
        let err = list_manager.swap("3").unwrap_err();
        assert_eq!(
            err.downcast_ref::<ListManagerError>(),
            Some(&ListManagerError::IndexOutOfRange(vec![1, 2, 3]))
        );
    }

    #[test]
    fn test_cut_with_out_of_index_input_and_multiple_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3, 4, 5], vec![1, 2]];
        let err = list_manager.swap("4").unwrap_err();
        assert_eq!(
            err.downcast_ref::<ListManagerError>(),
            Some(&ListManagerError::IndexOutOfRange(vec![1, 2]))
        );
    }

    #[test]
    fn test_cut_with_negative_input() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3]];
        let err = list_manager.swap("-1").unwrap_err();
        assert_eq!(
            err.downcast_ref::<ListManagerError>(),
            Some(&ListManagerError::InvalidInput("-1".to_string()))
        );
    }

    #[test]
    fn test_swap_with_1_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3, 4, 5, 6]];
        let expected_list = vec![vec![5, 6, 4, 3, 2, 1]];
        list_manager.swap("3").unwrap();
        assert_eq!(list_manager.list, expected_list);
    }

    #[test]
    fn test_swap_with_2_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![9, 8, 7, 6], vec![5, 4, 3, 2, 1]];
        let expected_list = vec![vec![7, 6, 8, 9], vec![3, 2, 1, 4, 5]];
        list_manager.swap("1").unwrap();
        assert_eq!(list_manager.list, expected_list);
    }
}
