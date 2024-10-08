use crate::types::Item;

#[derive(Debug, Clone, PartialEq)]
pub struct Miner {
    pub name: String,
    pub speed: usize,
    pub output: Item,
}

impl Miner {
    pub fn new(item: Item) -> Self {
        Miner {
            name: "Miner".to_string(),
            speed: 60,
            output: item,
        }
    }
}
