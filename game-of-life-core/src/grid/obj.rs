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
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn grid_default_test() {
        const WIDTH: usize = 4;
        const HEIGHT: usize = 4;
        let mut grid = GameOfLifeGrid::<WIDTH, HEIGHT>::default();
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
    proptest! {
        #[test]
        fn activation_test(x in 0usize..4, y in 0usize..4) {
            let mut grid = GameOfLifeGrid::<4, 4>::default();
            let r = grid.grid[y][x];
            assert_eq!(Cell::Dead, r);
            let r = grid.activate(x, y);
            assert!(r.is_ok());
            let r = grid.grid[y][x];
            assert_eq!(Cell::Alive, r);
        }
    }
}
