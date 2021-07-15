use ggez::{conf, GameResult};
use simple_logger::SimpleLogger;

mod settings;

use settings::*;
use snek_ai::AiPlayer;
use snek_core::players::Move;
use snek_core::players::{human::player::HumanPlayer, Player};
use snek_core::{
    entities::grid::Grid,
    game::{self, state::GameState},
};

fn main() -> GameResult {
    // Turn on logging
    SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .init()
        .unwrap();

    // Create a new ggez Context and EventLoop
    let (ctx, event_loop) = ggez::ContextBuilder::new(settings::GAME_TITLE, "Rufus Stone")
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

    // Create an appropriate player for the current game mode
    let player: Box<dyn Move> = match game_mode {
        game::mode::Mode::OnePlayer(Player::Human) => Box::new(HumanPlayer::default()),
        _ => Box::new(AiPlayer::default()),
    };

    // Create a proper prng
    let prng = rand::thread_rng();

    // Create a default grid
    let grid = Grid::default();

    // Create a new GameState
    let game_state = GameState::new(grid, prng, player);

    // Start the game!
    ggez::event::run(ctx, event_loop, game_state);
}
