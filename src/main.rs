use ggez::{ContextBuilder, GameResult};
mod components;  // Declare components as a module
mod systems;     // Declare systems as a module
mod resources;     // Declare resources as a module

use resources::game_state::Game;                // Import Game from components

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("camera_follow", "Victor")
        .window_setup(ggez::conf::WindowSetup::default().title("Camera Follow Example"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()?;
    
    let game = Game::new();
    ggez::event::run(ctx, event_loop, game)
}