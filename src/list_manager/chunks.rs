use super::{cerror::ListManagerError, ctype::ReturnType, list_manager::ListManager};
use std::cmp;

impl ListManager {
    pub fn chunks(&mut self, s: &str) -> ReturnType {
        if self.list.len() == 0 {
            return Err(Box::new(ListManagerError::NoAvailableListToPerformAction));
        }
        if s.len() == 0 {
            return Err(Box::new(ListManagerError::NoInput));
        }
        let chunk_size = s
            .trim()
            .parse::<usize>()
            .map_err(|_| ListManagerError::InvalidInput(s.to_string()))?;

        let new_list = self.list.iter().fold(vec![], |mut acc, l| {
            let mut i = 0;
            while i < l.len() {
                let chunk = Vec::from(&l[i..cmp::min(i + chunk_size, l.len())]);
                acc.push(chunk);
                i += chunk_size
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
    fn test_chunks_with_no_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![];
        let err = list_manager.chunks("1").unwrap_err();
        assert_eq!(
            err.downcast_ref::<ListManagerError>(),
            Some(&ListManagerError::NoAvailableListToPerformAction)
        );
    }

    #[test]
    fn test_chunks_with_no_input() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3]];
        let err = list_manager.chunks("").unwrap_err();
        assert_eq!(
            err.downcast_ref::<ListManagerError>(),
            Some(&ListManagerError::NoInput)
        );
    }

    #[test]
    fn test_chunks_with_invalid_input() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3]];
        let err = list_manager.chunks("no").unwrap_err();
        assert_eq!(
            err.downcast_ref::<ListManagerError>(),
            Some(&ListManagerError::InvalidInput("no".to_string()))
        );
    }

    #[test]
    fn test_chunck_with_1_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3]];
        let expected_list = vec![vec![1, 2], vec![3]];
        list_manager.chunks("2").unwrap();
        assert_eq!(list_manager.list, expected_list);
    }

    #[test]
    fn test_chunck_with_2_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let expected_list = vec![vec![1, 2], vec![3], vec![4, 5], vec![6]];
        list_manager.chunks("2").unwrap();
        assert_eq!(list_manager.list, expected_list);
    }

    #[test]
    fn test_chunck_with_3_list() {
        let mut list_manager = ListManager::new();
        list_manager.list = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12]];
        let expected_list = vec![
            vec![1, 2, 3],
            vec![4, 5],
            vec![6, 7, 8],
            vec![9, 10],
            vec![11, 12],
        ];
        list_manager.chunks("3").unwrap();
        assert_eq!(list_manager.list, expected_list);
    }
}
