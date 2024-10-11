use byteworks::game::components::GameState;
use byteworks::app;

fn main() -> std::io::Result<()> {
    app::run(&mut GameState::new())?;

    ratatui::restore();

    Ok(())
}
