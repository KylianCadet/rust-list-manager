use std::fmt::{Display, Write};

use super::{r#struct::ListManager, r#type::ReturnType};

impl Display for ListManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list_iter = self.list.iter();

        if let Some(v) = list_iter.next() {
            f.write_fmt(format_args!("{:?}", v)).unwrap();
            for v in list_iter {
                f.write_char('\n').unwrap();
                f.write_fmt(format_args!("{:?}", v)).unwrap();
            }
        }

        Ok(())
    }
}

impl ListManager {
    pub fn display(&self) -> ReturnType {
        println!("{}", self);
        Ok(None)
    }
}
