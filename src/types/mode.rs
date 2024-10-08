#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Normal,
    Build,
    Delete,
}

impl Mode {
    pub fn get_mode(&self) -> String {
        match self {
            Mode::Normal => "Normal".to_string(),
            Mode::Build => "Build".to_string(),
            Mode::Delete => "Delete".to_string(),
        }
    }
}
