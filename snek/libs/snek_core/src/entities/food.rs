use rand::{Rng, RngCore};

use crate::types::Vec2;

use super::snek::Snek;

#[derive(Debug, PartialEq)]
pub struct Food(Vec2);

impl Food {
    /// Create a new Food at the specified location
    pub fn new(pos: Vec2) -> Self {
        Self(pos)
    }

    /// Create a new Food at a random location, but not on top of the Snek
    pub fn random(bounds: &Vec2, snek: &Snek, prng: &mut dyn RngCore) -> Self {
        let mut x = prng.gen_range(0..bounds.x);
        let mut y = prng.gen_range(0..bounds.y);

        let mut proposed_food = Vec2::new(x, y);

        while snek.touches(&proposed_food) {
            log::warn!(
                "proposed_food {:?} would be inside the Snek! Try again...",
                &proposed_food
            );
            x = prng.gen_range(0..bounds.x);
            y = prng.gen_range(0..bounds.y);
            proposed_food = Vec2::new(x, y);
        }

        log::warn!("New random Food: {:?}", &proposed_food);

        Self(proposed_food)
    }

    pub fn pos(&self) -> &Vec2 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use crate::entities::food::Food;

    use super::*;

    #[test]
    fn new_food() {
        let food = Food::new(Vec2::new(0, 0));

        assert_eq!(food.0, Vec2::new(0, 0));
    }

    #[test]
    fn random_food() {
        let mut prng = ChaCha8Rng::from_seed(Default::default());

        let bounds = Vec2::new(10, 10);
        let snek = Snek::default();
        let food = Food::random(&bounds, &snek, &mut prng);

        assert_eq!(food.0, Vec2::new(8, 9));
    }
}
