use std::io;

use super::{ctype::ReturnType, list_manager::ListManager};
use regex::Regex;

fn parse(arg: &str) -> Option<Vec<i8>> {
    let re = Regex::new(r"^\[(-?)\d+(,\s*(-?)\d+)*\]$").unwrap();

    if !re.is_match(arg) {
        return None;
    }

    Some(
        arg[1..arg.len() - 1]
            .split(',')
            .map(|s| s.trim().parse::<i8>().unwrap())
            .collect(),
    )
}

impl ListManager {
    pub fn define(&mut self, arg: &str) -> ReturnType {
        if let Some(parsed) = parse(arg) {
            self.list.push(parsed);
            Ok(None)
        } else {
            Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, arg)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_with_valid_input() {
        let input = "[9, 8, 7, 6, 5, 4, 3, 2, 1]";
        let expected_list: Vec<i8> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let result = parse(input);
        assert_eq!(result.unwrap(), expected_list);
    }

    #[test]
    fn test_parse_edge_case() {
        let input = "[1]";
        let expected_list: Vec<i8> = vec![1];
        let result = parse(input);
        assert_eq!(result.unwrap(), expected_list);
    }

    #[test]
    fn test_parse_negative_input() {
        let input = "[-1, -2]";
        let expected_list: Vec<i8> = vec![-1, -2];
        let result = parse(input);
        assert_eq!(result.unwrap(), expected_list);
    }

    #[test]
    fn test_parse_negative_positive_mixed_input() {
        let input = "[-1, 2, -2, 3, -2]";
        let expected_list: Vec<i8> = vec![-1, 2, -2, 3, -2];
        let result = parse(input);
        assert_eq!(result.unwrap(), expected_list);
    }

    #[test]
    fn test_define_with_empty_input() {
        let input = "";
        let result = parse(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_define_with_empty_brackets_input() {
        let input = "[]";
        let result = parse(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_define_with_invalid_input() {
        let input = "invalid";
        let result = parse(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_define_with_more_brackets_input() {
        let input = "[[1, 2]]";
        let result = parse(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_define_with_more_invalid_bracket_place_input() {
        let input = "]1, 2[";
        let result = parse(input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_define_with_more_invalid_number_input() {
        let input = "[1, 2, k, 2, 3]";
        let result = parse(input);
        assert_eq!(result, None);
    }
}
