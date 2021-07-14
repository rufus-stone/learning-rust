use std::ops::{Add, Sub};

pub mod direction {
    use std::f64::consts::FRAC_PI_2;

    pub const LEFT: f64 = FRAC_PI_2;
    pub const RIGHT: f64 = -FRAC_PI_2;
    pub const AHEAD: f64 = 0.0;
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    /// Create a new Vec2 with the specified x and y components
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Create a new Vec2 that is the result of rotating self by the specified angle
    pub fn rotate(&self, radians: f64) -> Self {
        let x1 = self.x as f64;
        let y1 = self.y as f64;

        let x2 = (radians.cos() * x1) - (radians.sin() * y1);
        let xy = (radians.sin() * x1) + (radians.cos() * y1);

        Self {
            x: x2 as i32,
            y: xy as i32,
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Add for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn addition() {
        // Create a new Vec2
        let v1 = Vec2::new(1, 0);
        let v2 = v1 + Vec2::new(-2, 1);

        assert_eq!(v2, Vec2::new(-1, 1));
    }

    #[test]
    fn subtraction() {
        // Create a new Vec2
        let v1 = Vec2::new(1, 0);
        let v2 = v1 - Vec2::new(-2, 1);

        assert_eq!(v2, Vec2::new(3, -1));
    }

    #[test]
    fn addition_ref() {
        // Create a new Vec2
        let v1 = &Vec2::new(1, 0);
        let v2 = v1 + &Vec2::new(-2, 1);

        assert_eq!(v2, Vec2::new(-1, 1));
    }

    #[test]
    fn subtraction_ref() {
        // Create a new Vec2
        let v1 = &Vec2::new(1, 0);
        let v2 = v1 - &Vec2::new(-2, 1);

        assert_eq!(v2, Vec2::new(3, -1));
    }

    #[test]
    fn rotation() {
        let up = Vec2::new(0, 1);

        let left = up.rotate(direction::LEFT);
        assert_eq!(left, Vec2::new(-1, 0));

        let right = up.rotate(direction::RIGHT);
        assert_eq!(right, Vec2::new(1, 0));

        let down = up.rotate(2.0 * direction::LEFT);
        assert_eq!(down, Vec2::new(0, -1));

        let down = up.rotate(2.0 * direction::RIGHT);
        assert_eq!(down, Vec2::new(0, -1));

        let same = up.rotate(direction::AHEAD);
        assert_eq!(same, up);
    }
}
