use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

use super::{food::Food, grid::Grid, snek::Snek, types::Vec2};

pub struct GameState<'a> {
    pub snek: Snek,
    pub food: Food,
    pub grid: Grid,
    pub prng: Box<dyn RngCore + 'a>,
    pub play: bool,
}

impl<'a> Default for GameState<'a> {
    fn default() -> Self {
        let mut prng = ChaCha8Rng::from_seed(Default::default());

        let grid = Grid::default();
        let snek = Snek::default();
        let food = Food::random(grid.bounds(), &snek, &mut prng);

        Self {
            snek,
            food,
            grid,
            prng: Box::new(prng),
            play: true,
        }
    }
}

impl<'a> std::fmt::Display for GameState<'a> {
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

        for chunk in tmp.chunks(self.grid.columns() as usize) {
            output = output + &chunk.concat() + "\n";
        }

        write!(f, "{}", output)
    }
}

impl<'a> GameState<'a> {
    /// Create a new GameState
    pub fn new(grid: Grid, prng: &'a mut dyn RngCore) -> Self {
        let snek = Snek::default();
        let food = Food::random(grid.bounds(), &snek, prng);

        Self {
            snek,
            food,
            grid,
            prng: Box::new(prng),
            play: true,
        }
    }

    /// Move the game forward one frame
    pub fn step(&mut self, direction: Vec2) {
        // Only step if the game is still in play
        if self.play {
            // First, update the Snek direction
            self.snek.set_direction(direction);

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
            println!("GAME OVER!!");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{snek::*, types::Vec2};

    use super::*;

    #[test]
    fn new_gamestate() {
        let grid = Grid::new(5, 5).unwrap();
        let mut prng = ChaCha8Rng::from_seed(Default::default());
        let state = GameState::new(grid, &mut prng);

        assert_eq!(state.grid.len(), 25);
        assert_eq!(state.snek.head(), &Vec2::new(0, 0));
        assert_eq!(state.food.pos(), &Vec2::new(4, 4));

        println!("{}", state);
    }

    #[test]
    fn snek_movement() {
        let mut state = GameState::default();

        println!("{}", state);

        // Play out a game
        state.step(LEFT);
        state.step(LEFT);
        state.step(UP);

        assert_eq!(state.snek.len(), 1);

        state.step(UP);
        state.step(UP);
        state.step(LEFT);
        state.step(LEFT);

        assert_eq!(state.snek.len(), 2);

        state.step(UP);
        state.step(UP);
        state.step(LEFT);
        state.step(LEFT);
        state.step(LEFT);
        state.step(LEFT);
        state.step(LEFT);

        assert_eq!(state.snek.len(), 3);

        state.step(UP);

        assert_eq!(state.snek.len(), 4);

        state.step(RIGHT);

        assert_eq!(state.snek.len(), 5);
        assert_eq!(state.play, true);

        state.step(DOWN);

        assert_eq!(state.snek.len(), 5);
        assert_eq!(state.play, false);

        println!("{}", state);
    }
}
