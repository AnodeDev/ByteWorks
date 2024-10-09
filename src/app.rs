use crate::display::Display;
use crate::types::{Machine, Miner, Mode, Type};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

use log::{error, info, warn};

pub struct GameState {
    pub build_menu: bool,
    pub config_menu: Option<Type>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            build_menu: false,
            config_menu: None,
        }
    }
}

pub fn run(display: &mut Display) -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(|frame| {
            display.draw(frame);
        })?;

        info!("{:?}", display.game_state.config_menu);

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match display.mode {
                    Mode::Normal => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('n') => {
                            if display.cursor.x_pos > 0 && display.game_state.config_menu == None {
                                display.cursor.x_pos -= 1;
                            }
                        }
                        KeyCode::Char('e') => {
                            if display.cursor.y_pos < display.size[1] - 1 && display.game_state.config_menu == None {
                                display.cursor.y_pos += 1;
                            }
                        }
                        KeyCode::Char('i') => {
                            if display.cursor.y_pos > 0 && display.game_state.config_menu == None {
                                display.cursor.y_pos -= 1;
                            }
                        }
                        KeyCode::Char('o') => {
                            if display.cursor.x_pos < display.size[0] - 1 && display.game_state.config_menu == None {
                                display.cursor.x_pos += 1;
                            }
                        }
                        KeyCode::Char('b') => {
                            display.mode = Mode::Build;
                            info!("Switched to Build mode");
                        }
                        KeyCode::Char('d') => {
                            display.mode = Mode::Delete;
                            info!("Switched to Delete mode");
                        }
                        KeyCode::Char('c') => {
                            if display.game_state.config_menu != None {
                                display.game_state.config_menu = None;
                            } else {
                                match &display.type_map[display.cursor.y_pos][display.cursor.x_pos]
                                {
                                    Type::Machine(machine) => {
                                        display.game_state.config_menu =
                                            Some(Type::Machine(machine.clone()));
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    },
                    Mode::Build => match key.code {
                        KeyCode::Esc => {
                            display.mode = Mode::Normal;
                            display.cursor.selected_type = Type::Nothing;
                            info!("Switched to Normal mode");
                        }
                        KeyCode::Char('n') => {
                            if display.cursor.x_pos > 0 {
                                display.cursor.x_pos -= 1;
                            }
                        }
                        KeyCode::Char('e') => {
                            if display.cursor.y_pos < display.size[1] - 1 {
                                display.cursor.y_pos += 1;
                            }
                        }
                        KeyCode::Char('i') => {
                            if display.cursor.y_pos > 0 {
                                display.cursor.y_pos -= 1;
                            }
                        }
                        KeyCode::Char('o') => {
                            if display.cursor.x_pos < display.size[0] - 1 {
                                display.cursor.x_pos += 1;
                            }
                        }
                        KeyCode::Char('m') => {
                            if let Event::Key(second_key) = event::read()? {
                                if key.kind == KeyEventKind::Press {
                                    match key.code {
                                        KeyCode::Char('m') => {
                                            if display.cursor.selected_type
                                                != Type::Machine(Machine::new())
                                            {
                                                display.cursor.selected_type =
                                                    Type::Machine(Machine::new());
                                            } else {
                                                display.cursor.selected_type = Type::Nothing;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        KeyCode::Enter => match &display.cursor.selected_type {
                            Type::Machine(machine) => display.place(
                                Type::Machine(machine.clone()),
                                [display.cursor.x_pos, display.cursor.y_pos],
                            )?,
                            _ => {}
                        },
                        _ => {}
                    },
                    Mode::Delete => match key.code {
                        KeyCode::Esc => {
                            display.mode = Mode::Normal;
                            info!("Switched to Normal mode");
                        }
                        KeyCode::Char('n') => {
                            if display.cursor.x_pos > 0 {
                                display.cursor.x_pos -= 1;
                            }
                        }
                        KeyCode::Char('e') => {
                            if display.cursor.y_pos < display.size[1] - 1 {
                                display.cursor.y_pos += 1;
                            }
                        }
                        KeyCode::Char('i') => {
                            if display.cursor.y_pos > 0 {
                                display.cursor.y_pos -= 1;
                            }
                        }
                        KeyCode::Char('o') => {
                            if display.cursor.x_pos < display.size[0] - 1 {
                                display.cursor.x_pos += 1;
                            }
                        }
                        KeyCode::Enter => {
                            display.place(
                                Type::Nothing,
                                [display.cursor.x_pos, display.cursor.y_pos],
                            )?;
                        }
                        _ => {}
                    },
                }
            }
        }
    }

    Ok(())
}
