use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::{
    entities::{food::Food, grid::Grid, snek::Snek},
    players::{human::player::HumanPlayer, Move},
    types::Vec2,
};

pub struct GameState<R, M>
where
    R: RngCore,
    M: Move,
{
    pub snek: Snek,
    pub food: Food,
    pub grid: Grid,
    pub prng: R,
    pub play: bool,
    pub player: M,
}

impl Default for GameState<ChaCha8Rng, HumanPlayer> {
    fn default() -> Self {
        let mut prng = ChaCha8Rng::from_seed(Default::default());

        let grid = Grid::default();
        let snek = Snek::default();
        let food = Food::random(grid.bounds(), &snek, &mut prng);

        let player = HumanPlayer::default();

        Self {
            snek,
            food,
            grid,
            prng,
            play: true,
            player,
        }
    }
}

impl<R, M> std::fmt::Display for GameState<R, M>
where
    R: RngCore,
    M: Move,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Bits
        let tmp: Vec<String> = self
            .grid
            .into_iter()
            .map(|pos| {
                // Check if pos is part of the snake
                if self.snek.head() == &pos {
                    "#".to_owned()
                } else if self.snek.parts().contains(&pos) {
                    "0".to_owned()
                } else if self.food.pos() == &pos {
                    "%".to_owned()
                } else {
                    ".".to_owned()
                }
            })
            .collect();

        let mut output = String::with_capacity((self.grid.len() + self.grid.rows()) as usize);
        output.push('\n');

        for chunk in tmp.chunks(self.grid.columns() as usize).rev() {
            output = output + &chunk.concat() + "\n";
        }

        write!(f, "{}", output)
    }
}

impl<R, M> GameState<R, M>
where
    R: RngCore,
    M: Move,
{
    /// Create a new GameState
    pub fn new(grid: Grid, mut prng: R, player: M) -> Self {
        let snek = Snek::default();
        let food = Food::random(grid.bounds(), &snek, &mut prng);

        Self {
            snek,
            food,
            grid,
            prng,
            play: true,
            player,
        }
    }

    /// Move the game forward one frame
    pub fn step(&mut self, orientation: Vec2) {
        // Only step if the game is still in play
        if self.play {
            // First, update the Snek direction
            self.snek.set_orientation(orientation);

            // Second, advance the Snek and reset the Food if it ate any
            match self.snek.advance(self.grid.bounds(), &self.food.pos()) {
                true => self.food = Food::random(self.grid.bounds(), &self.snek, &mut self.prng),
                false => (),
            }

            // Check for collisions with itself, and stop play if so
            if self.snek.hit_self() {
                self.play = false;
            }
        } else {
            log::warn!("GAME OVER!!");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        entities::snek::{FACING_DOWN, FACING_LEFT, FACING_RIGHT, FACING_UP},
        types::Vec2,
    };

    use super::*;

    #[test]
    fn new_gamestate() {
        let mut prng = ChaCha8Rng::from_seed(Default::default());

        let grid = Grid::new(5, 5).unwrap();
        let mut player = HumanPlayer::default();
        let state = GameState::new(grid, &mut prng, &mut player);

        assert_eq!(state.grid.len(), 25);
        assert_eq!(state.snek.head(), &Vec2::new(0, 0));
        assert_eq!(state.food.pos(), &Vec2::new(4, 4));

        println!("{}", state);
    }

    #[test]
    fn sample_game() {
        let mut prng = ChaCha8Rng::from_seed(Default::default());
        let grid = Grid::new(5, 5).unwrap();
        let mut player = HumanPlayer::default();

        let mut state = GameState::new(grid, &mut prng, &mut player);

        println!("{}", state);

        // Play out a game
        state.step(FACING_LEFT);
        state.step(FACING_LEFT);
        state.step(FACING_DOWN);
        assert_eq!(state.snek.len(), 1);

        state.step(FACING_DOWN);
        state.step(FACING_DOWN);
        state.step(FACING_LEFT);
        state.step(FACING_LEFT);
        assert_eq!(state.snek.len(), 2);

        state.step(FACING_LEFT);
        state.step(FACING_LEFT);
        state.step(FACING_LEFT);
        state.step(FACING_LEFT);
        state.step(FACING_DOWN);
        state.step(FACING_DOWN);
        assert_eq!(state.snek.len(), 3);

        state.step(FACING_LEFT);
        state.step(FACING_DOWN);
        assert_eq!(state.snek.len(), 4);

        state.step(FACING_RIGHT);
        state.step(FACING_UP);
        state.step(FACING_LEFT);
        assert_eq!(state.snek.hit_self(), true);

        println!("{}", state);
    }
}
