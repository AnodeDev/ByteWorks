use crate::types::State;

use std::collections::HashMap;

lazy_static! {
    pub static ref ITEMS: HashMap<String, Item> = {
        let mut items: HashMap<String, Item> = HashMap::new();
        items.insert(
            "Iron Ore".to_string(),
            Item::new(
            "Iron Ore".to_string(),
            State::Solid
        ));
        items.insert(
            "Iron Ingot".to_string(),
            Item::new(
            "Iron Ingot".to_string(),
            State::Solid
        ));
        items.insert(
            "Iron Plate".to_string(),
            Item::new(
            "Iron Plate".to_string(),
            State::Solid
        ));
        items.insert(
            "Iron Rod".to_string(),
            Item::new(
            "Iron Rod".to_string(),
            State::Solid
        ));

        items
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub name: String,
    pub state: State,
}

impl Item {
    pub fn new(name: String, state: State) -> Self {
        Item { name, state }
    }
}
