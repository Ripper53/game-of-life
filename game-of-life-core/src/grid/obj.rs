use super::{Cell, CellOutOfBoundsError, Grid};

pub struct GameOfLifeGrid<const WIDTH: usize, const HEIGHT: usize> {
    grid: [[Cell; HEIGHT]; WIDTH],
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for GameOfLifeGrid<WIDTH, HEIGHT> {
    fn default() -> Self {
        GameOfLifeGrid {
            grid: [[Cell::Dead; HEIGHT]; WIDTH],
        }
    }
}
impl<const WIDTH: usize, const HEIGHT: usize> Grid for GameOfLifeGrid<WIDTH, HEIGHT> {
    fn get(&self, x: usize, y: usize) -> Result<Cell, CellOutOfBoundsError> {
        if let Some(cells) = self.grid.get(y) {
            if let Some(cell) = cells.get(x) {
                Ok(*cell)
            } else {
                Err(CellOutOfBoundsError::new(x, y))
            }
        } else {
            Err(CellOutOfBoundsError::new(x, y))
        }
    }
    fn activate(&mut self, x: usize, y: usize) -> Result<(), CellOutOfBoundsError> {
        if let Some(cells) = self.grid.get_mut(y) {
            if let Some(cell) = cells.get_mut(x) {
                *cell = Cell::Alive;
                Ok(())
            } else {
                Err(CellOutOfBoundsError::new(x, y))
            }
        } else {
            Err(CellOutOfBoundsError::new(x, y))
        }
    }
    fn width(&self) -> usize {
        WIDTH
    }
    fn height(&self) -> usize {
        HEIGHT
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use crate::tests::*;

    #[test]
    fn grid_default_test() {
        let mut grid = TestGrid::default();
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let r = grid.get(x, y);
                assert!(r.is_ok());
                let r = r.unwrap();
                assert_eq!(Cell::Dead, r);
                let direct_r = grid.grid[y][x];
                assert_eq!(direct_r, r);
            }
        }
    }
    #[test]
    fn grid_size_test() {
        let grid = TestGrid::default();
        assert_eq!(WIDTH, grid.width());
        assert_eq!(HEIGHT, grid.height());
    }
    proptest! {
        #[test]
        fn activation_test(x in 0usize..WIDTH, y in 0usize..HEIGHT) {
            let mut grid = TestGrid::default();
            let r = grid.grid[y][x];
            assert_eq!(Cell::Dead, r);
            let r = grid.activate(x, y);
            assert!(r.is_ok());
            let r = grid.grid[y][x];
            assert_eq!(Cell::Alive, r);
        }
    }
}
