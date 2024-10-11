use crate::modes::Mode;

pub struct GameState {
    pub mode: Mode,
}

impl GameState {
    pub fn new() -> Self {
        Self { mode: Mode::Normal }
    }
}
