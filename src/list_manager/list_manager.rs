pub struct ListManager {
    pub list: Vec<Vec<i8>>,
}

impl ListManager {
    pub fn new() -> ListManager {
        ListManager { list: vec![] }
    }
}
