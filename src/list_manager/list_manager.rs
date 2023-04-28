use super::ctype::ReturnType;

pub struct ListManager {
    list: Box<[u8]>,
}

impl ListManager {
    pub fn new() -> ListManager {
        ListManager { list: Box::new([]) }
    }

    pub fn unknown(&self, function_name: &str) -> ReturnType {
        println!("[unimplemented] {}", function_name);
        Ok(None)
    }
}
