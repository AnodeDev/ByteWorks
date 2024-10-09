#[macro_use]
extern crate lazy_static;

pub mod app;
pub mod cursor;
pub mod display;
pub mod types;
pub mod utils;

use crate::utils::logging;

fn main() -> std::io::Result<()> {
    logging::init_logging();

    let map_size = [120, 60];
    let mut display = display::Display::new(map_size);

    app::run(&mut display)?;

    ratatui::restore();

    Ok(())
}
