use std::collections::VecDeque;

use rand::{Rng, RngCore};

use crate::game::types::Vec2;

use super::grid::Grid;

pub const UP: Vec2 = Vec2 { x: 0, y: -1 };
pub const DOWN: Vec2 = Vec2 { x: 0, y: 1 };
pub const LEFT: Vec2 = Vec2 { x: -1, y: 0 };
pub const RIGHT: Vec2 = Vec2 { x: 1, y: 0 };

#[derive(Debug)]
pub struct Snek {
    parts: VecDeque<Vec2>,
    direction: Vec2,
}

impl Default for Snek {
    fn default() -> Self {
        Self {
            parts: VecDeque::from(vec![Vec2::new(0, 0)]),
            direction: Vec2::new(0, 1), // Sneks start moving straight up
        }
    }
}

impl Snek {
    /// Create a new Snek at the specified position
    pub fn new(pos: Vec2) -> Self {
        Self {
            parts: VecDeque::from(vec![pos]),
            direction: Vec2::new(0, 1), // Sneks start moving straight up
        }
    }

    /// How long is the Snek (not counting its head)
    pub fn len(&self) -> usize {
        self.parts.len() - 1
    }

    /// Get an immutable ref to the parts of the Snek
    pub fn parts(&self) -> Vec<Vec2> {
        self.parts.iter().copied().collect::<Vec<Vec2>>()
    }

    /// Get an immutable ref to the current position of the Snek head
    pub fn head(&self) -> &Vec2 {
        self.parts.back().unwrap()
    }

    /// Get an immutable ref to the current direction of the Snek
    pub fn direction(&self) -> &Vec2 {
        &self.direction
    }

    /// Set the direction of the Snek
    pub fn set_direction(&mut self, new_direction: Vec2) {
        self.direction = new_direction;
    }

    /// Turn the Snek to the left
    pub fn turn_left(&mut self) {
        self.direction.x = (self.direction.x as f32).sin() as i32;
        self.direction.y = (self.direction.x as f32).sin() as i32;
    }

    /// Turn the Snek to the right
    pub fn turn_right(&mut self) {
        self.direction.x = (self.direction.x + 1) % 2;
        self.direction.y = (self.direction.y + 1) % 2;
    }

    /// Set a random direction for the Snek
    pub fn random_direction(&mut self, prng: &mut dyn RngCore) {
        // First, flip a coin to decide if we're moving horizontally or vertically, and another for forwards and backwards
        match (prng.gen::<bool>(), prng.gen::<bool>()) {
            (true, true) => self.set_direction(Vec2::new(-1, 0)),
            (true, false) => self.set_direction(Vec2::new(0, -1)),
            (false, true) => self.set_direction(Vec2::new(1, 0)),
            (false, false) => self.set_direction(Vec2::new(0, 1)),
        }
    }

    /// Check if any part of the Snek is touching the specified position
    pub fn touches(&self, pos: &Vec2) -> bool {
        self.parts.contains(pos)
    }

    /// Check if the Snek head is touching the specified position
    pub fn head_is_touching(&self, pos: &Vec2) -> bool {
        self.head() == pos
    }

    /// Check if the Snek head has hit the Snek body
    pub fn hit_self(&self) -> bool {
        // Find the first position in the VecDeque where the body_part is in the same place as the head
        // This will always find a match, as the head is part of the body, so eventually the head will be compared with the head...
        // But, as the head is always the last element in the VecDeque, this is fine
        let p = self
            .parts
            .iter()
            .position(|body_part| body_part == self.head())
            .unwrap();

        // If the head is NOT touching the body, then p will be the position of the head, i.e. the last element (or self.parts.len() - 1)
        // So, if p is less than the last element, then it must be hitting the body
        p < self.parts.len() - 1
    }

    /// Move the Snek
    pub fn advance(&mut self, bounds: &Vec2, food: &Vec2) -> bool {
        // Check where the Snek wants to go, wrapping around if it crosses the bounds of the grid
        let new_head = Grid::wrap(bounds, &(self.head() + &self.direction));

        // Add the new position
        self.parts.push_back(new_head);

        // Did the Snek eat the food?
        if self.head_is_touching(food) {
            println!("Ate the food!");
            return true;
        } else {
            // If not, pop off the last bit of the tail
            self.parts.pop_front();
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use crate::game::food::Food;

    use super::*;

    #[test]
    fn default_snek() {
        let snek = Snek::default();

        assert_eq!(snek.head(), &Vec2::new(0, 0));
        assert_eq!(snek.direction(), &Vec2::new(0, 1));
    }

    #[test]
    fn new_snek() {
        let snek = Snek::new(Vec2::new(5, 5));

        assert_eq!(snek.head(), &Vec2::new(5, 5));
        assert_eq!(snek.direction(), &Vec2::new(0, 1));
    }

    #[test]
    fn snek_advance() {
        let mut prng = ChaCha8Rng::from_seed(Default::default());

        let bounds = Vec2::new(10, 10);
        let mut snek = Snek::default();
        let food = Food::random(&bounds, &snek, &mut prng);

        snek.advance(&bounds, food.pos());

        assert_eq!(snek.head(), &Vec2::new(0, 1));

        snek.set_direction(LEFT);
        snek.advance(&bounds, food.pos());

        assert_eq!(snek.head(), &Vec2::new(9, 1));

        snek.advance(&bounds, food.pos());
        assert_eq!(snek.head(), &Vec2::new(8, 1));
    }

    #[test]
    fn snek_turning() {
        let mut prng = ChaCha8Rng::from_seed(Default::default());

        let bounds = Vec2::new(10, 10);
        let mut snek = Snek::default();
        let food = Food::random(&bounds, &snek, &mut prng);

        let mut x: f64 = 0.0;
        for i in 1..10 {
            println!("{}", x.sin());
            x = i as f64 * 0.5;
        }

        assert_eq!(snek.direction(), &Vec2::new(0, 1));

        snek.turn_left();
        assert_eq!(snek.direction(), &Vec2::new(-1, 0));

        snek.turn_left();
        assert_eq!(snek.direction(), &Vec2::new(0, -1));

        snek.turn_left();
        assert_eq!(snek.direction(), &Vec2::new(1, 0));

        snek.turn_left();
        assert_eq!(snek.direction(), &Vec2::new(0, 1));
    }
}
