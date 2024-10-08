use crate::types::Recipe;

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Machine {
    pub name: String,
    pub recipe: Option<Recipe>,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            name: "Machine".to_string(),
            recipe: None,
        }
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut recipe = String::new();

        if let Some(current_recipe) = &self.recipe {
            recipe = current_recipe.to_string();
        } else {
            recipe = "No recipe selected...".to_string();
        }

        write!(f, "NAME: {}\nRECIPE: {}", self.name, recipe)
    }
}
