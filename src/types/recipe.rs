use crate::types::Item;

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Recipe {
    pub name: String,
    pub input: Vec<Item>,
    pub output: Vec<Item>,
}

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let input: String = self.input.iter().map(|item| item.name.clone()).collect::<Vec<String>>().join(", ");
        let output: String = self.output.iter().map(|item| item.name.clone()).collect::<Vec<String>>().join(", ");

        write!(f, "{}\n\nINPUT: {}\nOUTPUT: {}", self.name, input, output)
    }
}
