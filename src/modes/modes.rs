use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};

use std::fmt;

pub enum Mode {
    Normal,
    Build,
    Delete,
}

impl Mode {
    pub fn handle_key_press(&self, key: &KeyEvent) -> KeyResult {
        if let Some(result) = self.movement_keys(key) {
            return result;
        }

        match self {
            Mode::Build => {
                match key.code {
                    KeyCode::Esc => KeyResult::ModeSwitch(Mode::Normal),
                    _ => KeyResult::Nop,
                }
            },
            Mode::Delete => {
                match key.code {
                    KeyCode::Esc => KeyResult::ModeSwitch(Mode::Normal),
                    _ => KeyResult::Nop,
                }
            },
            Mode::Normal => {
                match key.code {
                    KeyCode::Char('q') => KeyResult::Quit,
                    KeyCode::Char('b') => KeyResult::ModeSwitch(Mode::Build),
                    KeyCode::Char('d') => KeyResult::ModeSwitch(Mode::Delete),
                    _ => KeyResult::Nop,
                }
            }
        }
    }

    fn movement_keys(&self, key: &KeyEvent) -> Option<KeyResult> {
        match key.code {
            KeyCode::Char('n') => Some(KeyResult::MoveCursor((1, 0), Direction::Negative)),
            KeyCode::Char('e') => Some(KeyResult::MoveCursor((0, 1), Direction::Positive)),
            KeyCode::Char('i') => Some(KeyResult::MoveCursor((0, 1), Direction::Negative)),
            KeyCode::Char('o') => Some(KeyResult::MoveCursor((1, 0), Direction::Positive)),
            _ => None,
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mode::Normal => write!(f, "NORMAL"),
            Mode::Build => write!(f, "BUILD"),
            Mode::Delete => write!(f, "DELETE"),
        }
    }
}

pub enum KeyResult {
    Nop,
    ModeSwitch(Mode),
    MoveCursor((usize, usize), Direction),
    Quit,
}

pub enum Direction {
    Positive,
    Negative,
}
