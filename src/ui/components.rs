use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::{Constraint, Layout, Stylize};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
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
}

trait PanelComponents {
    fn format(&mut self) -> Text;
    fn render(&mut self, frame: &mut Frame);
}

#[derive(Clone)]
pub struct Size {
    width: u16,
    height: u16,
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
    title: String,
    header: Option<Line<'a>>,
    content: Vec<Line<'a>>,
    panel_type: PanelType,
}

impl<'a> Panel<'a> {
    pub fn new(size: Size, title: String, header: Option<Line<'a>>, content: Vec<Line<'a>>, panel_type: PanelType) -> Self {
        Panel {
            size,
            title,
            header,
            content,
            panel_type,
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
        };



        frame.render_widget(Clear, header);
        frame.render_widget(Clear, area);

        frame.render_widget(
            Paragraph::new(Line::styled("ByteWorks", Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD)))
                .alignment(Alignment::Center)
                .block(Block::bordered()
                    .border_type(BorderType::Rounded)
                ),
            header,
        );

        frame.render_widget(
            Paragraph::new(formatted_content)
                .block(Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title(self.title.clone())
                ),
            area,
        );
    }
}
