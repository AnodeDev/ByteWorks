use crate::types::{Item, ITEMS, State};

use std::fmt;

lazy_static! {
    pub static ref RECIPES: Vec<Recipe > = {
        let mut recipes = Vec::new();

        recipes.push(Recipe::new(
            "Iron Plates".to_string(),
            vec![ITEMS["Iron Ingot"].clone()],
            vec![ITEMS["Iron Plate"].clone()],
        ));

        recipes
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct Recipe {
    pub name: String,
    pub input: Vec<Item>,
    pub output: Vec<Item>,
}

impl Recipe {
    fn new(name: String, input: Vec<Item>, output: Vec<Item>) -> Self {
        Recipe {
            name,
            input,
            output,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let input: String = self
            .input
            .iter()
            .map(|item| item.name.clone())
            .collect::<Vec<String>>()
            .join(", ");
        let output: String = self
            .output
            .iter()
            .map(|item| item.name.clone())
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "{}\n\nINPUT: {}\nOUTPUT: {}", self.name, input, output)
    }
}
