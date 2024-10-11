use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use ratatui::text::Line;
use ratatui::style::{Color, Modifier, Style};

use crate::modes::{Mode, KeyResult};
use crate::game::components::GameState;
use crate::ui;

pub fn run(game_state: &mut GameState) -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let size = ui::Size::new(120, 60);

    loop {
        let mut map_panel = ui::Panel::new(
            size.clone(), 
            "MAP".to_string(),
            None,
            vec![Line::from("No map yet...")],
            ui::PanelType::Map,
        );

        let mut left_panel = ui::Panel::new(
            size.clone(),
            "INFO".to_string(),
            Some(Line::styled("Game Info", Style::default().add_modifier(Modifier::UNDERLINED))),
            vec![Line::from(format!("Mode: {}", game_state.mode))],
            ui::PanelType::SidePanel(ui::Side::Left),
        );

        let mut right_panel = ui::Panel::new(
            size.clone(),
            "TILE".to_string(),
            Some(Line::styled("Tile Info", Style::default().add_modifier(Modifier::UNDERLINED))),
            vec![Line::from("No tile info yet...")],
            ui::PanelType::SidePanel(ui::Side::Right),
        );

        terminal.draw(|frame| {
            map_panel.display(frame);
            left_panel.display(frame);
            right_panel.display(frame);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                let result = game_state.mode.handle_key_press(&key);

                match result {
                    KeyResult::Quit => break,
                    KeyResult::ModeSwitch(mode) => game_state.mode = mode,
                    _ => {},
                }
            }
        }
    }

    Ok(())
}
