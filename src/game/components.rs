use crate::modes::Mode;
use crate::ui::Size;
use crate::game::entities::Asset;

use ratatui::text::{Line, Span};
use ratatui::style::{Style, Color, Modifier};
use ratatui::layout::Alignment;

#[derive(Clone)]
pub enum TileType {
    Empty,
    Asset(Asset),
}

impl TileType {
    pub fn list_info(&self) -> Vec<Line> {
        match self {
            TileType::Empty => vec![
                Line::styled("Empty Tile", Style::default().add_modifier(Modifier::UNDERLINED).add_modifier(Modifier::BOLD)).alignment(Alignment::Center),
                Line::from(""),
                Line::from("Nothing but a patch of grass").alignment(Alignment::Center),
            ],
            TileType::Asset(asset) => asset.display_info(),
        }
    }
}

pub struct GameState {
    pub mode: Mode,
}

impl GameState {
    pub fn new() -> Self {
        Self { mode: Mode::Normal }
    }
}

pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl Cursor {
    fn new() -> Self {
        Cursor {
            x: 0,
            y: 0,
        }
    }
}

#[derive(Clone)]
pub struct Tile {
    tile_type: TileType,
    char: char,
}

impl Tile {
    fn new(tile_type: TileType, char: char) -> Self {
        Tile {
            tile_type,
            char,
        }
    }
}

pub struct Map {
    pub map: Vec<Vec<Tile>>,
    pub cursor: Cursor,
} 

impl Map {
    pub fn new(size: Size) -> Self {
        let mut map: Vec<Vec<Tile>> = Vec::new();

        for _ in 0..size.height {
            let mut row: Vec<Tile> = Vec::new();
            for _ in 0..size.width {
                row.push(Tile::new(TileType::Empty, ' '));
            }

            map.push(row);
        }

        Map {
            map,
            cursor: Cursor::new(),
        }
    }

    pub fn display(&self) -> Vec<Line> {
        let mut map: Vec<Line> = Vec::new();

        for (y, row) in self.map.clone().iter().enumerate() {
            let mut spans: Vec<Span> = Vec::new();

            for (x, tile) in row.iter().enumerate() {
                if x == self.cursor.x && y == self.cursor.y {
                    spans.push(Span::styled(tile.char.to_string(), Style::default().fg(Color::Black).bg(Color::White)));
                } else {
                    spans.push(Span::raw(tile.char.to_string()));
                }
            }

            map.push(Line::from(spans));
        }

        map
    }

    pub fn place(&mut self, tile_type: TileType, coords: (usize, usize)) {
        let char = match tile_type.clone() {
            TileType::Empty => ' ',
            TileType::Asset(asset) => format!("{}", asset.asset_type).chars().nth(0).unwrap(),
        };

        let tile = Tile::new(tile_type, char);

        self.map[coords.1][coords.0] = tile;
    }

    pub fn get_tile_info(&self) -> TileType {
        self.map[self.cursor.y][self.cursor.x].tile_type.clone()
    }
}
