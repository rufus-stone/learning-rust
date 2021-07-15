use ggez::event::EventHandler;
use ggez::{Context, GameError, GameResult};
use rand::RngCore;

use crate::players::Move;

use super::state::GameState;

impl<R, M> EventHandler<GameError> for GameState<R, M>
where
    R: RngCore,
    M: Move,
{
    /// Called every frame
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Where is the Snek currently facing?
        let current_orientation = *self.snek.orientation();

        // Check to see if the player has made a new move, otherwise continue in the current direction
        if let Some(new_orientation) = self.player.make_move(ctx) {
            self.step(new_orientation);
        } else {
            self.step(current_orientation);
        }

        // Finally, check if the the game has ended, and quit if so
        if !self.play {
            log::warn!("{}", self);
            ggez::event::quit(ctx);
        }

        Ok(())
    }

    /// Draw the game screen
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
}
