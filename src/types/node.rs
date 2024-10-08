use crate::types::Item;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub name: String,
    pub item: Item,
}

impl Node {
    pub fn new(name: String, item: Item) -> Self {
        Node {
            name,
            item,
        }
    }
}
