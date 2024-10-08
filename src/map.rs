use crate::cursor::Cursor;
use crate::types::{Mode, Type};

use ratatui::widgets::{Block, BorderType, Paragraph};
use ratatui::prelude::{Constraint, Layout, Stylize};
use ratatui::text::{Text, Line, Span};
use ratatui::style::{Style, Color};

use log::{info, warn, error};

pub struct Map {
    pub map: Vec<Vec<String>>,
    pub type_map: Vec<Vec<Type>>,
    pub size: [usize; 2],
    pub cursor: Cursor,
    pub mode: Mode,
}

impl Map {
    pub fn new(size: [usize; 2]) -> Self {
        let mut map: Vec<Vec<String>> = Vec::new();
        let mut type_map: Vec<Vec<Type>> = Vec::new();
        let cursor = Cursor::new();

        for y in 0..size[1] {
            let mut row: Vec<String> = Vec::new();
            let mut type_row: Vec<Type> = Vec::new();

            for x in 0..size[0] {
                row.push(" ".to_string());
                type_row.push(Type::Nothing);
            }

            map.push(row);
            type_map.push(type_row);
        }

        Map {
            map,
            type_map,
            size,
            cursor,
            mode: Mode::Normal,
        }
    }

    pub fn place(&mut self, nodetype: Type, coords: [usize; 2]) -> std::io::Result<()> {
        let mut first_letter = String::new();

        if nodetype != Type::Nothing {
            first_letter = nodetype.get_type().chars().nth(0).unwrap().to_string();
        } else {
            first_letter = " ".to_string();
        }

        if coords[0] < self.size[0] && coords[1] < self.size[1] {
            self.map[coords[1]][coords[0]] = first_letter;
            self.type_map[coords[1]][coords[0]] = nodetype;
        } 

        Ok(())
    }

    fn map_to_lines(&self) -> Vec<Line<'_>> {
        let mut map_lines: Vec<Line> = Vec::new();

        for (y, row) in self.map.iter().enumerate() {
            let mut spans: Vec<Span> = Vec::new();

            for (x, c) in row.iter().enumerate() {
                if x == self.cursor.x_pos && y == self.cursor.y_pos {
                    spans.push(Span::styled(c, Style::default().fg(Color::Black).bg(Color::White)));
                } else {
                    spans.push(Span::styled(c, Style::default()));
                }
            }

            map_lines.push(Line::from(spans));
        }

        map_lines
    }

    fn display_info(&self) -> String {
        match self.mode {
            Mode::Normal => return "MODE: NORMAL".into(),
            Mode::Build => {
                let selected = self.cursor.selected_type.get_type();
                return format!("MODE: BUILD\nSELECTED: {}", selected);
            },
            Mode::Delete => return "MODE: DELETE".into(),
        }
    }

    fn display_tile(&self) -> String {
        let mut tile_info = String::new();
        let tile_type = &self.type_map[self.cursor.y_pos][self.cursor.x_pos];

        match tile_type {
            Type::Machine(machine) => tile_info = format!("{}", machine),
            Type::Nothing => tile_info = format!("Just some grass..."),
            _ => {},
        }

        info!("({}, {}) - {}", &self.cursor.x_pos, &self.cursor.y_pos, &tile_info);

        format!("{}\n\n{}", tile_type.get_type(), tile_info)
    }

    pub fn draw(&self, frame: &mut ratatui::Frame) {
        let [info_unpadded, _, main_unpadded, _, tile_unpadded] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(self.size[0] as u16 + 2),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(frame.area());

        let [_, info_area, _] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(info_unpadded);

        let [_, main_area, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(self.size[1] as u16 + 2),
            Constraint::Fill(1),
        ])
        .areas(main_unpadded);

        let [_, tile_area, _] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(tile_unpadded);

        frame.render_widget(
            Paragraph::new(self.display_info())
                .block(
                    Block::bordered()
                        .border_type(BorderType::Rounded)
                        .title("Info")
                ),
            info_area,
        );

        frame.render_widget(
            Paragraph::new(self.map_to_lines())
                .block(
                    Block::bordered()
                        .border_type(BorderType::Rounded)
                        .title("Map")
                ),
            main_area,
        );

        frame.render_widget(
            Paragraph::new(self.display_tile())
                .block(
                    Block::bordered()
                        .border_type(BorderType::Rounded)
                        .title("Tile")
                ),
            tile_area,
        );
    }
}
