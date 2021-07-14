use settings::HUMAN;
use simple_logger::SimpleLogger;

mod settings;

use snek_core::{
    game::state::GameState,
    ggez::{self, conf, ContextBuilder, GameResult},
};

fn main() -> GameResult {
    // Turn on logging
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    // Create a new ggez Context and EventLoop
    let (ctx, event_loop) = ContextBuilder::new(settings::GAME_TITLE, "Rufus Stone")
        .window_setup(
            conf::WindowSetup::default()
                .title(settings::GAME_TITLE)
                .vsync(true),
        )
        .window_mode(
            conf::WindowMode::default().dimensions(settings::SCREEN_WIDTH, settings::SCREEN_HEIGHT),
        )
        .build()
        .unwrap();

    // Either start training the AI or playing the game
    let game_mode = HUMAN;

    // Create a proper prng
    //let mut prng = rand::thread_rng();

    // Create a new GameState
    let mut game_state = GameState::default();

    // Start the game!
    ggez::event::run(ctx, event_loop, game_state);
}
