use super::r#type::ReturnType;

pub struct ListManager {
    pub list: Vec<Vec<i8>>,
}

impl ListManager {
    pub fn new() -> ListManager {
        ListManager { list: vec![] }
    }

    pub fn help(&self) -> ReturnType {
        println!(
            "add NUMBER\tIncreases every element of the previous list(s) by the given amount."
        );
        println!(
            "chunks CHUNK_SIZE\tSeparates the previous list(s) in chunks of the given amount."
        );
        println!("cut INDEX\tTakes an index as parameter and splits the previous list(s) at the given index.");
        println!("define ARRAY\tAdds a list, ARRAY must be in the following format: [1,2,3]");
        println!("display\tDisplay the list(s)");
        println!("exit\tExit the program");
        println!("flatten\tUnifies the previous lists as one.");
        println!("help\tDisplay this message");
        println!("sort\tSorts the previous list(s) in increasing order.");
        println!("sum\tPerforms the cumulated sum of the previous list if there is only one list, and consumes it.");
        println!("swap\tTakes an index as parameter and swaps the elements of the previous list(s) using the index as a swapping point.");
        Ok(None)
    }
}
