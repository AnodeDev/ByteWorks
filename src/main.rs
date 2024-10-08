pub mod app;
pub mod map;
pub mod cursor;
pub mod types;
pub mod utils;

use crate::utils::logging;

fn main() -> std::io::Result<()> {
    logging::init_logging();

    let map_size = [120, 60];
    let mut map = map::Map::new(map_size);

    app::run(&mut map)?;

    ratatui::restore();

    Ok(())
}
