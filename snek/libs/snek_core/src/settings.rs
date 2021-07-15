use crate::{game::mode::Mode, players::Player};

pub const GAME_TITLE: &str = "Snek";

pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;

pub const HUMAN: Mode = Mode::OnePlayer(Player::Human);
pub const COMPUTER: Mode = Mode::OnePlayer(Player::Computer);
pub const TRAIN_AI: Mode = Mode::TrainAi(Player::Computer);

#[derive(Clone, Debug)]
pub struct Config {
    pub eye_photoreceptors: usize,
    pub brain_neurons: usize,
    pub outputs: usize,
    pub generation_length: usize,
    pub population_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            eye_photoreceptors: 5, // Paddle Y, Ball X, Ball Y, Ball VX, Ball VY
            brain_neurons: 15,
            outputs: 1,            // Whether the move the paddle up or down
            generation_length: 10, // How many serves to play for
            population_size: 100,  // How big should the pool of trainee AIs be
        }
    }
}
