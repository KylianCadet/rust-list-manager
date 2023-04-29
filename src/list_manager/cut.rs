use super::{r#error::ListManagerError, r#struct::ListManager, r#type::ReturnType};

impl ListManager {
    pub fn cut(&mut self, s: &str) -> ReturnType {
        if self.list.is_empty() {
            return Err(Box::new(ListManagerError::NoAvailableListToPerformAction));
        }
        if s.is_empty() {
            return Err(Box::new(ListManagerError::NoInput));
        }
        let cut_index_value = s
            .trim()
            .parse::<usize>() // u8 because input cannot be negative
            .map_err(|_| ListManagerError::InvalidInput(s.to_string()))?;

        for l in self.list.iter() {
            if cut_index_value > l.len() - 1 {
                return Err(Box::new(ListManagerError::IndexOutOfRange(l.to_vec())));
            }
        }

        let new_list = self
            .list
            .iter()
            .fold(vec![], |mut acc: Vec<Vec<i8>>, curr| {
                let (a, b) = curr.split_at(cut_index_value + 1);
                if !a.is_empty() {
                    acc.push(a.to_vec())
                };
                if !b.is_empty() {
                    acc.push(b.to_vec());
                }
                acc
            });

        self.list = new_list;
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
        let err = list_manager.cut("1").unwrap_err();
        assert_eq!(
            err.downcast_ref::<ListManagerError>(),
            Some(&ListManagerError::NoAvailableListToPerformAction)
        );
    }

    #[test]
    fn test_cut_with_no_input() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3]];
        let err = list_manager.cut("").unwrap_err();
        assert_eq!(
            err.downcast_ref::<ListManagerError>(),
            Some(&ListManagerError::NoInput)
        );
    }

    #[test]
    fn test_cut_with_invalid_input() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3]];
        let err = list_manager.cut("no").unwrap_err();
        assert_eq!(
            err.downcast_ref::<ListManagerError>(),
            Some(&ListManagerError::InvalidInput("no".to_string()))
        );
    }

    #[test]
    fn test_cut_with_out_of_index_input() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3]];
        let err = list_manager.cut("3").unwrap_err();
        assert_eq!(
            err.downcast_ref::<ListManagerError>(),
            Some(&ListManagerError::IndexOutOfRange(vec![1, 2, 3]))
        );
    }

    #[test]
    fn test_cut_with_out_of_index_input_and_multiple_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3, 4, 5], vec![1, 2]];
        let err = list_manager.cut("4").unwrap_err();
        assert_eq!(
            err.downcast_ref::<ListManagerError>(),
            Some(&ListManagerError::IndexOutOfRange(vec![1, 2]))
        );
    }

    #[test]
    fn test_cut_with_negative_input() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3]];
        let err = list_manager.cut("-1").unwrap_err();
        assert_eq!(
            err.downcast_ref::<ListManagerError>(),
            Some(&ListManagerError::InvalidInput("-1".to_string()))
        );
    }

    #[test]
    fn test_cut_with_1_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3]];
        let expected_list = vec![vec![1, 2], vec![3]];
        list_manager.cut("1").unwrap();
        assert_eq!(list_manager.list, expected_list);
    }

    #[test]
    fn test_cut_with_2_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]];
        let expected_list = vec![vec![1, 2, 3, 4], vec![5], vec![6, 7, 8, 9], vec![10]];
        list_manager.cut("3").unwrap();
        assert_eq!(list_manager.list, expected_list);
    }

    #[test]
    fn test_cut_with_3_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12]];
        let expected_list = vec![
            vec![1, 2],
            vec![3, 4, 5],
            vec![6, 7],
            vec![8, 9, 10],
            vec![11, 12],
        ];
        list_manager.cut("1").unwrap();
        assert_eq!(list_manager.list, expected_list);
    }
}
