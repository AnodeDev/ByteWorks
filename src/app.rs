use crate::map::Map;
use crate::types::{Type, Mode, Machine, Miner};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

use log::{info, warn, error};

pub struct GameState {
    pub build_menu: bool,
    pub config_menu: bool,
}

pub fn run(map: &mut Map) -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(|frame| {
            map.draw(frame);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match map.mode {
                    Mode::Normal => {
                        match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Char('n') => {
                                if map.cursor.x_pos > 0 {
                                    map.cursor.x_pos -= 1;
                                }
                            },
                            KeyCode::Char('e') => {
                                if map.cursor.y_pos < map.size[1] - 1 {
                                    map.cursor.y_pos += 1;
                                }
                            },
                            KeyCode::Char('i') => {
                                if map.cursor.y_pos > 0 {
                                    map.cursor.y_pos -= 1;
                                }
                            },
                            KeyCode::Char('o') => {
                                if map.cursor.x_pos < map.size[0] - 1 {
                                    map.cursor.x_pos += 1;
                                }
                            },
                            KeyCode::Char('b') => {
                                map.mode = Mode::Build;
                                info!("Switched to Build mode");
                            },
                            KeyCode::Char('d') => {
                                map.mode = Mode::Delete;
                                info!("Switched to Delete mode");
                            },
                            _ => {},
                        }
                    },
                    Mode::Build => {
                        match key.code {
                            KeyCode::Esc => {
                                map.mode = Mode::Normal;
                                map.cursor.selected_type = Type::Nothing;
                                info!("Switched to Normal mode");
                            },
                            KeyCode::Char('n') => {
                                if map.cursor.x_pos > 0 {
                                    map.cursor.x_pos -= 1;
                                }
                            },
                            KeyCode::Char('e') => {
                                if map.cursor.y_pos < map.size[1] - 1 {
                                    map.cursor.y_pos += 1;
                                }
                            },
                            KeyCode::Char('i') => {
                                if map.cursor.y_pos > 0 {
                                    map.cursor.y_pos -= 1;
                                }
                            },
                            KeyCode::Char('o') => {
                                if map.cursor.x_pos < map.size[0] - 1 {
                                    map.cursor.x_pos += 1;
                                }
                            },
                            KeyCode::Char('m') => {
                                if let Event::Key(second_key) = event::read()? {
                                    if key.kind == KeyEventKind::Press {
                                        match key.code {
                                            KeyCode::Char('m') => {
                                                if map.cursor.selected_type != Type::Machine(Machine::new()) {
                                                    map.cursor.selected_type = Type::Machine(Machine::new());
                                                } else {
                                                    map.cursor.selected_type = Type::Nothing;
                                                }
                                            },
                                            _ => {},
                                        }
                                    }
                                }
                            },
                            KeyCode::Enter => {
                                match &map.cursor.selected_type {
                                    Type::Machine(machine) => map.place(Type::Machine(machine.clone()), [map.cursor.x_pos, map.cursor.y_pos])?,
                                    _ => {},
                                }

                                info!("({}, {}) - {}", &map.cursor.x_pos, &map.cursor.y_pos, &map.cursor.selected_type.get_type());
                            },
                            _ => {},
                        }
                    },
                    Mode::Delete => {
                        match key.code {
                            KeyCode::Esc => {
                                map.mode = Mode::Normal;
                                info!("Switched to Normal mode");
                            },
                            KeyCode::Char('n') => {
                                if map.cursor.x_pos > 0 {
                                    map.cursor.x_pos -= 1;
                                }
                            },
                            KeyCode::Char('e') => {
                                if map.cursor.y_pos < map.size[1] - 1 {
                                    map.cursor.y_pos += 1;
                                }
                            },
                            KeyCode::Char('i') => {
                                if map.cursor.y_pos > 0 {
                                    map.cursor.y_pos -= 1;
                                }
                            },
                            KeyCode::Char('o') => {
                                if map.cursor.x_pos < map.size[0] - 1 {
                                    map.cursor.x_pos += 1;
                                }
                            },
                            KeyCode::Enter => {
                                map.place(Type::Nothing, [map.cursor.x_pos, map.cursor.y_pos])?;
                            },
                            _ => {},
                        }
                    },
                }
            }
        }
    }

    Ok(())
}
