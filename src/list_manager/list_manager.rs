use super::ctype::ReturnType;

pub struct ListManager {
    pub list: Vec<Vec<i8>>,
}

impl ListManager {
    pub fn new() -> ListManager {
        ListManager { list: vec![] }
    }

    pub fn unknown(&self, function_name: &str) -> ReturnType {
        println!("[unimplemented] {}", function_name);
        Ok(None)
    }
}
