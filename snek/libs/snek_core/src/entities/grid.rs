use crate::types::Vec2;

#[derive(Debug)]
pub struct Grid(Vec2);

impl Grid {
    /// Create a new Grid with the specified width and height
    /// This must have a width and height of at least 1 - no empty or 1-dimensional grids allowed!
    pub fn new(width: usize, height: usize) -> Option<Self> {
        if width == 0 || height == 0 {
            None
        } else {
            Some(Self(Vec2::new(width as i32, height as i32)))
        }
    }

    /// Get an immutable ref to the internal Vec2 that describes the boundsof the Grid
    pub fn bounds(&self) -> &Vec2 {
        &self.0
    }

    /// Get the area the Grid
    pub fn len(&self) -> usize {
        self.0.x as usize * self.0.y as usize
    }

    /// This should never be true
    /// Just including to silence Clippy warning about a len() function without a corresponding is_empty() function!
    pub fn is_empty(&self) -> bool {
        self.0 == Vec2::default()
    }

    /// Get the width of the Grid
    pub fn width(&self) -> usize {
        self.0.x as usize
    }

    /// Get the height of the Grid
    pub fn height(&self) -> usize {
        self.0.y as usize
    }

    /// Get the number of columns in the Grid
    pub fn columns(&self) -> usize {
        self.0.x as usize
    }

    /// Get the number of rows in the Grid
    pub fn rows(&self) -> usize {
        self.0.y as usize
    }

    /// Get a Vec2 describing the centre position in the Grid
    pub fn centre(&self) -> Vec2 {
        Vec2::new(self.0.x / 2, self.0.y / 2)
    }

    /// Wrap the specified position so that it fits within the specified bounds
    pub fn wrap(bounds: &Vec2, pos: &Vec2) -> Vec2 {
        let x = match pos.x % bounds.x < 0 {
            true => bounds.x + (pos.x % bounds.x),
            false => pos.x % bounds.x,
        };

        let y = match pos.y % bounds.y < 0 {
            true => bounds.y + (pos.y % bounds.y),
            false => pos.y % bounds.y,
        };

        Vec2::new(x, y)
    }

    /// Get the
    pub fn xy_at_index(&self, idx: usize) -> Option<Vec2> {
        // Ignore index values that are outside the grid
        if idx >= self.width() * self.height() {
            None
        } else {
            let x = idx % self.width();
            let y = idx / self.height();

            Some(Vec2::new(x as i32, y as i32))
        }
    }

    pub fn index_at_xy(&self, xy: Vec2) -> Option<usize> {
        // Ignore x and y positions that are outside the grid
        if xy.x >= self.width() as i32 || xy.y >= self.height() as i32 {
            None
        } else {
            let idx = (xy.x + (self.width() as i32 * xy.y)) as usize;
            Some(idx)
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
                let xy = self.grid.xy_at_index(self.pos).unwrap();
                self.pos += 1;
                Some(xy)
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

        assert_eq!(grid.index_at_xy(Vec2::new(0, 0)), Some(0));
        assert_eq!(grid.index_at_xy(Vec2::new(4, 0)), Some(4));
        assert_eq!(grid.index_at_xy(Vec2::new(0, 1)), Some(5));
        assert_eq!(grid.index_at_xy(Vec2::new(4, 4)), Some(24));
        assert_eq!(grid.index_at_xy(Vec2::new(4, 5)), None);
        assert_eq!(grid.index_at_xy(Vec2::new(5, 0)), None);
        assert_eq!(grid.index_at_xy(Vec2::new(0, 5)), None);
        assert_eq!(grid.index_at_xy(Vec2::new(5, 6)), None);

        assert_eq!(grid.xy_at_index(0), Some(Vec2::new(0, 0)));
        assert_eq!(grid.xy_at_index(10), Some(Vec2::new(0, 2)));
        assert_eq!(grid.xy_at_index(17), Some(Vec2::new(2, 3)));
        assert_eq!(grid.xy_at_index(24), Some(Vec2::new(4, 4)));
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
            log::info!("{:?}", pos);
        }
    }
}
