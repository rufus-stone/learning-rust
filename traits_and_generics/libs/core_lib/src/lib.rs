use rand::{Rng, RngCore};

use ggez::event::EventHandler;
use ggez::{Context, GameError, GameResult};

pub trait Move {
    fn make_move(&mut self) -> u8;
}

impl Move for Box<dyn Move> {
    fn make_move(&mut self) -> u8 {
        self.as_mut().make_move()
    }
}

pub struct GameState<M, R>
where
    M: Move,
    R: RngCore,
{
    player: M,
    prng: R,
}

impl<M, R> GameState<M, R>
where
    M: Move,
    R: RngCore,
{
    pub fn new(player: M, prng: R) -> Self {
        Self { player, prng }
    }
}

impl<M, R> EventHandler<GameError> for GameState<M, R>
where
    M: Move,
    R: RngCore,
{
    /// Called every frame
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let random_choice = self.prng.gen_range(0..5);
        let player_move = self.player.make_move();
        println!("{}: Move: {}", random_choice, player_move);
        Ok(())
    }

    /// Draw the game screen
    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
}
