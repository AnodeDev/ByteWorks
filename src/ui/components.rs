use ratatui::Frame;
use ratatui::prelude::{Constraint, Layout};
use ratatui::layout::Alignment;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, Clear, Paragraph};

#[derive(Clone)]
pub enum Side {
    Left,
    Right,
}

#[derive(Clone)]
pub enum PanelType {
    Map,
    SidePanel(Side),
    Title,
}

trait PanelComponents {
    fn format(&mut self) -> Text;
    fn render(&mut self, frame: &mut Frame);
}

#[derive(Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
        Size {
            width,
            height,
        }
    }
}

#[derive(Clone)]
pub struct Panel<'a> {
    size: Size,
    title: Option<String>,
    header: Option<Line<'a>>,
    content: Vec<Line<'a>>,
    panel_type: PanelType,
    pub alignment: Alignment,
}

impl<'a> Panel<'a> {
    pub fn new(size: Size, title: Option<String>, header: Option<Line<'a>>, content: Vec<Line<'a>>, panel_type: PanelType) -> Self {
        Panel {
            size,
            title,
            header,
            content,
            panel_type,
            alignment: Alignment::Left,
        }
    }

    pub fn display(&mut self, frame: &mut Frame) {
        self.render(frame);
    }
}

impl<'a> PanelComponents for Panel<'a> {
    fn format(&mut self) -> Text {
        let mut formatted_content = Vec::<Line>::new();

        if let Some(header) = &self.header {
            formatted_content.push(header.clone());
            formatted_content.push(Line::from("\n"));
        }

        for line in &self.content {
            formatted_content.push(line.clone());
        }

        Text::from(formatted_content)
    }

    fn render(&mut self, frame: &mut Frame) {
        let mut binding = self.clone();
        let formatted_content = binding.format();

        let [header, _, unpadded, _, _] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(self.size.height + 2),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
            .areas(frame.area());

        let area = match &self.panel_type {
            PanelType::Map => {
                let [_, _, area, _, _] = Layout::horizontal([
                    Constraint::Fill(1),
                    Constraint::Length(1),
                    Constraint::Length(self.size.width + 2),
                    Constraint::Length(1),
                    Constraint::Fill(1),
                ])
                    .areas(unpadded);

                area
            },
            PanelType::SidePanel(side) => {
                match &side {
                    Side::Left => {
                        let [area, _, _, _, _] = Layout::horizontal([
                            Constraint::Fill(1),
                            Constraint::Length(1),
                            Constraint::Length(self.size.width + 2),
                            Constraint::Length(1),
                            Constraint::Fill(1),
                        ])
                            .areas(unpadded);

                        area
                    },
                    Side::Right => {
                        let [_, _, _, _, area] = Layout::horizontal([
                            Constraint::Fill(1),
                            Constraint::Length(1),
                            Constraint::Length(self.size.width + 2),
                            Constraint::Length(1),
                            Constraint::Fill(1),
                        ])
                            .areas(unpadded);

                        area
                    },
                }
            },
            PanelType::Title => {
                header
            },
        };

        frame.render_widget(Clear, area);

        match self.title.clone() {
            Some(title) => {
                frame.render_widget(
                    Paragraph::new(formatted_content)
                        .alignment(self.alignment)
                        .block(Block::bordered()
                            .border_type(BorderType::Rounded)
                            .title(title)
                        ),
                    area,
                );
            },
            None => {
                frame.render_widget(
                    Paragraph::new(formatted_content)
                        .alignment(self.alignment)
                        .block(Block::bordered()
                            .border_type(BorderType::Rounded)
                        ),
                    area,
                );
            },
        }
    }
}
