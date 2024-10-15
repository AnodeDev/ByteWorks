use ratatui::crossterm::event::{self, Event, KeyEventKind};
use ratatui::text::Line;
use ratatui::style::{Modifier, Style};
use ratatui::layout::Alignment;

use crate::modes::{KeyResult, Direction};
use crate::game::components::{GameState, Map, TileType};
use crate::game::entities::{Asset, AssetType, Item, Recipe};
use crate::ui;

pub fn run(game_state: &mut GameState) -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let size = ui::Size::new(120, 60);
    let mut map = Map::new(size.clone());
    let iron_ore_recipe = Recipe::new(None, vec![ Item::IronOre ]);
    let iron_plate_recipe = Recipe::new(Some(vec![ Item::IronIngot ]), vec![ Item::IronPlates ]);
    let rein_iron_plate_recipe = Recipe::new(Some(vec![ Item::Screws, Item::IronPlates ]), vec![ Item::ReinforcedIronPlates ]);
    let iron_node = Asset::new(AssetType::Node, iron_ore_recipe);
    let iron_plate_constructor = Asset::new(AssetType::Constructor, iron_plate_recipe);
    let rein_iron_plate_assembler = Asset::new(AssetType::Assembler, rein_iron_plate_recipe);

    map.place(TileType::Asset(iron_node), (10, 5));
    map.place(TileType::Asset(iron_plate_constructor), (20, 10));
    map.place(TileType::Asset(rein_iron_plate_assembler), (30, 15));

    loop {
        let tile_info = map.get_tile_info();

        let mut title_panel = ui::Panel::new(
            size.clone(),
            None,
            None,
            vec![ Line::from("ByteWorks") ],
            ui::PanelType::Title,
        );

        let mut map_panel = ui::Panel::new(
            size.clone(), 
            Some("MAP".to_string()),
            None,
            map.display(),
            ui::PanelType::Map,
        );

        let mut left_panel = ui::Panel::new(
            size.clone(),
            Some("INFO".to_string()),
            Some(Line::styled("Game Info", Style::default().add_modifier(Modifier::UNDERLINED))),
            vec![Line::from(format!("Mode: {}", game_state.mode))],
            ui::PanelType::SidePanel(ui::Side::Left),
        );

        let mut right_panel = ui::Panel::new(
            size.clone(),
            Some("TILE".to_string()),
            None,
            tile_info.list_info(),
            ui::PanelType::SidePanel(ui::Side::Right),
        );

        title_panel.alignment = Alignment::Center;

        terminal.draw(|frame| {
            title_panel.display(frame);
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
                    KeyResult::MoveCursor((x, y), direction) => {
                        match direction {
                            Direction::Positive => {
                                if x != 0 && map.cursor.x < size.width as usize - 1 {
                                    map.cursor.x += 1;
                                } else if y != 0 && map.cursor.y < size.height as usize - 1 {
                                    map.cursor.y += 1;
                                }
                            },
                            Direction::Negative => {
                                if x != 0 && map.cursor.x > 0 {
                                    map.cursor.x -= 1;
                                } else if y != 0 && map.cursor.y > 0 {
                                    map.cursor.y -= 1;
                                }
                            },
                        }
                    },
                    _ => {},
                }
            }
        }
    }

    Ok(())
}
