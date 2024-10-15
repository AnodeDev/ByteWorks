use ratatui::text::Line;
use ratatui::style::{Style, Modifier};
use ratatui::layout::Alignment;

use std::fmt;

trait AssetComponent {
    fn get_info(&self) -> Vec<Line>;
}

pub enum Purity {
    Impure,
    Normal,
    Pure,
}

impl fmt::Display for Purity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Purity::Impure => write!(f, "Impure"),
            Purity::Normal => write!(f, "Normal"),
            Purity::Pure => write!(f, "Pure"),
        }
    }
}

#[derive(Clone)]
pub enum Item {
    IronOre,
    CopperOre,
    IronIngot,
    CopperIngot,
    IronPlates,
    IronRods,
    Screws,
    ReinforcedIronPlates,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Item::IronOre => write!(f, "Iron Ore"),
            Item::CopperOre => write!(f, "Copper Ore"),
            Item::IronIngot => write!(f, "Iron Ingot"),
            Item::CopperIngot => write!(f, "Copper Ingot"),
            Item::IronPlates => write!(f, "Iron Plates"),
            Item::IronRods => write!(f, "Iron Rods"),
            Item::Screws => write!(f, "Screws"),
            Item::ReinforcedIronPlates => write!(f, "Reinforced Iron Plates"),
        }
    }
}

#[derive(Clone)]
pub enum AssetType {
    Node,
    Constructor,
    Assembler,
}

impl fmt::Display for AssetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AssetType::Node => write!(f, "Node"),
            AssetType::Constructor => write!(f, "Constructor"),
            AssetType::Assembler => write!(f, "Assembler"),
        }
    }
}

#[derive(Clone)]
pub struct Recipe {
    input: Option<Vec<Item>>,
    output: Vec<Item>,
}

impl Recipe {
    pub fn new(input: Option<Vec<Item>>, output: Vec<Item>) -> Self {
        Recipe {
            input,
            output,
        }
    }
}

#[derive(Clone)]
pub struct Asset {
    pub asset_type: AssetType,
    recipe: Recipe,
}

impl Asset {
    pub fn new(asset_type: AssetType, recipe: Recipe) -> Self {
        Asset {
            asset_type,
            recipe,
        }
    }

    pub fn display_info(&self) -> Vec<Line> {
        self.get_info()
    }
}

impl AssetComponent for Asset {
    fn get_info(&self) -> Vec<Line> {
        let output = self.recipe.output.clone().iter().map(|item| item.to_string()).collect::<Vec<String>>();

        let mut info: Vec<Line> = Vec::new();

        info.push(Line::styled(self.asset_type.to_string(), Style::default().add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED)).alignment(Alignment::Center));
        info.push(Line::from(""));
        
        match self.recipe.input.clone() {
            Some(input) => {
                info.push(Line::styled("Input", Style::default().add_modifier(Modifier::UNDERLINED)).alignment(Alignment::Center));
                for item in input {
                    info.push(Line::from(item.to_string()).alignment(Alignment::Center));
                }
                info.push(Line::from(""));
            },
            None => {},
        }

        info.push(Line::styled("Output", Style::default().add_modifier(Modifier::UNDERLINED)).alignment(Alignment::Center));
        for item in self.recipe.output.clone() {
            info.push(Line::from(item.to_string()).alignment(Alignment::Center));
        }

        info
    }
}
