use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
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
}
