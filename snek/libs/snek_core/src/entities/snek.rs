use std::collections::VecDeque;

use crate::types::{direction, Vec2};

use super::grid::Grid;

pub const FACING_UP: Vec2 = Vec2 { x: 0, y: -1 };
pub const FACING_DOWN: Vec2 = Vec2 { x: 0, y: 1 };
pub const FACING_LEFT: Vec2 = Vec2 { x: -1, y: 0 };
pub const FACING_RIGHT: Vec2 = Vec2 { x: 1, y: 0 };

#[derive(Debug)]
pub struct Snek {
    parts: VecDeque<Vec2>,
    orientation: Vec2,
}

impl Default for Snek {
    /// A default Snek starts are grid position 0,0 facing up
    fn default() -> Self {
        log::warn!("New default Snek");
        Self {
            parts: VecDeque::from(vec![Vec2::new(0, 0)]),
            orientation: FACING_UP, // Sneks start facing straight up
        }
    }
}

impl Snek {
    /// Create a new Snek at the specified position
    pub fn new(pos: Vec2) -> Self {
        log::warn!("New custom Snek");
        Self {
            parts: VecDeque::from(vec![pos]),
            orientation: FACING_UP, // Sneks start facing straight up
        }
    }

    /// How long is the Snek (not counting its head)
    pub fn len(&self) -> usize {
        self.parts.len() - 1
    }

    /// Is the Snek empty (i.e. is it just a head with no tail)
    pub fn is_empty(&self) -> bool {
        self.parts.len() == 1
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
    pub fn orientation(&self) -> &Vec2 {
        &self.orientation
    }

    /// Set the direction of the Snek
    pub fn set_orientation(&mut self, new_orientation: Vec2) {
        self.orientation = new_orientation;
    }

    /// Turn the Snek to the left
    pub fn turn_left(&mut self) {
        self.orientation = self.orientation.rotate(direction::LEFT);
    }

    /// Turn the Snek to the right
    pub fn turn_right(&mut self) {
        self.orientation = self.orientation.rotate(direction::RIGHT);
    }

    /// Turn the Snek
    pub fn turn(&mut self, radians: f64) {
        self.orientation = self.orientation.rotate(radians);
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
        // If the Snek is only a head then it can't have hit itself!
        if self.is_empty() {
            return false;
        }

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
        let new_head = Grid::wrap(bounds, &(self.head() + &self.orientation));

        // Add the new position
        self.parts.push_back(new_head);

        // Did the Snek eat the food?
        if self.head_is_touching(food) {
            log::warn!("Ate the food!");
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

    use crate::entities::food::Food;

    use super::*;

    #[test]
    fn default_snek() {
        let snek = Snek::default();

        assert_eq!(snek.head(), &Vec2::new(0, 0));
        assert_eq!(snek.orientation(), &Vec2::new(0, 1));
    }

    #[test]
    fn new_snek() {
        let snek = Snek::new(Vec2::new(5, 5));

        assert_eq!(snek.head(), &Vec2::new(5, 5));
        assert_eq!(snek.orientation(), &Vec2::new(0, 1));
    }

    #[test]
    fn snek_advance() {
        let mut prng = ChaCha8Rng::from_seed(Default::default());

        let bounds = Vec2::new(10, 10);
        let mut snek = Snek::default();
        let food = Food::random(&bounds, &snek, &mut prng);

        snek.advance(&bounds, food.pos());

        assert_eq!(snek.head(), &Vec2::new(0, 1));

        snek.set_orientation(FACING_LEFT);
        snek.advance(&bounds, food.pos());

        assert_eq!(snek.head(), &Vec2::new(9, 1));

        snek.advance(&bounds, food.pos());
        assert_eq!(snek.head(), &Vec2::new(8, 1));
    }

    #[test]
    fn snek_turning() {
        // Create a new default Snek
        let mut snek = Snek::default();

        // Default Sneks start facing up
        assert_eq!(snek.orientation(), &Vec2::new(0, 1)); // Up

        snek.turn_left();
        assert_eq!(snek.orientation(), &Vec2::new(-1, 0)); // Left

        snek.turn_left();
        assert_eq!(snek.orientation(), &Vec2::new(0, -1)); // Down

        snek.turn_left();
        assert_eq!(snek.orientation(), &Vec2::new(1, 0)); // Right

        snek.turn_left();
        assert_eq!(snek.orientation(), &Vec2::new(0, 1)); // Up

        snek.turn_right();
        assert_eq!(snek.orientation(), &Vec2::new(1, 0)); // Right

        snek.turn_right();
        assert_eq!(snek.orientation(), &Vec2::new(0, -1)); // Down

        snek.turn_right();
        assert_eq!(snek.orientation(), &Vec2::new(-1, 0)); // Left

        snek.turn_right();
        assert_eq!(snek.orientation(), &Vec2::new(0, 1)); // Up
    }
}
