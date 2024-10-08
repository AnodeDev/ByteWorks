use crate::types::Type;

pub struct Cursor {
    pub x_pos: usize,
    pub y_pos: usize,
    pub selected_type: Type,
}

impl Cursor {
    pub fn new() -> Self {
        Cursor {
            x_pos: 10,
            y_pos: 5,
            selected_type: Type::Nothing,
        }
    }
}
