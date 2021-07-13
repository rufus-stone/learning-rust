use std::ops::Index;

use super::types::Vec2;

#[derive(Debug)]
pub struct Grid(Vec2);

impl Grid {
    pub fn new(width: u32, height: u32) -> Option<Self> {
        if width == 0 || height == 0 {
            None
        } else {
            Some(Self(Vec2::new(width as i32, height as i32)))
        }
    }

    pub fn bounds(&self) -> &Vec2 {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.x as usize * self.0.y as usize
    }

    pub fn width(&self) -> u32 {
        self.0.x() as u32
    }

    pub fn height(&self) -> u32 {
        self.0.y() as u32
    }

    pub fn columns(&self) -> usize {
        self.0.x() as usize
    }

    pub fn rows(&self) -> usize {
        self.0.y() as usize
    }

    pub fn centre(&self) -> Vec2 {
        Vec2::new(self.0.x() / 2, self.0.y() / 2)
    }

    pub fn wrap(bounds: &Vec2, pos: &Vec2) -> Vec2 {
        let x = match pos.x() % bounds.x() < 0 {
            true => bounds.x() + (pos.x() % bounds.x()),
            false => pos.x() % bounds.x(),
        };

        let y = match pos.y() % bounds.y() < 0 {
            true => bounds.y() + (pos.y() % bounds.y()),
            false => pos.y() % bounds.y(),
        };

        Vec2::new(x, y)
    }

    pub fn xy_at_index(&self, idx: u32) -> Option<(u32, u32)> {
        // Ignore index values that are outside the grid
        if idx >= self.width() * self.height() {
            None
        } else {
            let x = idx % self.width();
            let y = idx / self.height();

            Some((x, y))
        }
    }

    pub fn index_at_xy(&self, x: u32, y: u32) -> Option<u32> {
        // Ignore x and y positions that are outside the grid
        if x >= self.width() || y >= self.height() {
            None
        } else {
            Some(x + (self.width() * y))
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self(Vec2::new(10, 10))
    }
}

pub struct GridIter<'a> {
    grid: &'a Grid,
    pos: usize,
}

impl<'a> Iterator for GridIter<'a> {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pos < self.grid.len() as usize {
            true => {
                let (x, y) = self.grid.xy_at_index(self.pos as u32).unwrap();
                self.pos += 1;
                Some(Vec2::new(x as i32, y as i32))
            }
            false => None,
        }
    }
}

impl<'a> IntoIterator for &'a Grid {
    type Item = Vec2;
    type IntoIter = GridIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GridIter {
            grid: &self,
            pos: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_xy_calcs() {
        // Create a new 5x5 Grid
        let grid = Grid::new(5, 5).unwrap();

        assert_eq!(grid.index_at_xy(0, 0), Some(0));
        assert_eq!(grid.index_at_xy(4, 0), Some(4));
        assert_eq!(grid.index_at_xy(0, 1), Some(5));
        assert_eq!(grid.index_at_xy(4, 4), Some(24));
        assert_eq!(grid.index_at_xy(4, 5), None);
        assert_eq!(grid.index_at_xy(5, 0), None);
        assert_eq!(grid.index_at_xy(0, 5), None);
        assert_eq!(grid.index_at_xy(5, 6), None);

        assert_eq!(grid.xy_at_index(0), Some((0, 0)));
        assert_eq!(grid.xy_at_index(10), Some((0, 2)));
        assert_eq!(grid.xy_at_index(17), Some((2, 3)));
        assert_eq!(grid.xy_at_index(24), Some((4, 4)));
        assert_eq!(grid.xy_at_index(25), None);
        assert_eq!(grid.xy_at_index(26), None);
    }

    #[test]
    fn wrap_test() {
        let bounds = Vec2::new(5, 5);

        let pos = Vec2::new(0, 0);
        assert_eq!(Grid::wrap(&bounds, &pos), Vec2::new(0, 0));

        let pos = Vec2::new(4, 5);
        assert_eq!(Grid::wrap(&bounds, &pos), Vec2::new(4, 0));

        let pos = Vec2::new(6, 6);
        assert_eq!(Grid::wrap(&bounds, &pos), Vec2::new(1, 1));

        let pos = Vec2::new(-2, 6);
        assert_eq!(Grid::wrap(&bounds, &pos), Vec2::new(3, 1));

        let pos = Vec2::new(-3, -1);
        assert_eq!(Grid::wrap(&bounds, &pos), Vec2::new(2, 4));
    }

    #[test]
    fn grid_iterator() {
        // Create a new 5x5 Grid
        let grid = Grid::new(5, 5).unwrap();

        for pos in &grid {
            println!("{:?}", pos);
        }
    }
}
