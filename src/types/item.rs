use crate::types::State;

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub name: String,
    pub state: State,
}

impl Item {
    pub fn new(name: String, state: State) -> Self {
        Item {
            name,
            state,
        }
    }
}
