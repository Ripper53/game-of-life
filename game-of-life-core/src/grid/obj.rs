use super::{ActivateCellOutOfBoundsError, Grid};

pub struct GameOfLifeGrid<const WIDTH: usize, const HEIGHT: usize> {
    grid: [[Cell; HEIGHT]; WIDTH],
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Cell {
    Dead,
    Alive,
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for GameOfLifeGrid<WIDTH, HEIGHT> {
    fn default() -> Self {
        GameOfLifeGrid {
            grid: [[Cell::Dead; HEIGHT]; WIDTH],
        }
    }
}
impl<const WIDTH: usize, const HEIGHT: usize> Grid for GameOfLifeGrid<WIDTH, HEIGHT> {
    fn activate(&mut self, x: usize, y: usize) -> Result<(), ActivateCellOutOfBoundsError> {
        if let Some(cells) = self.grid.get_mut(y) {
            if let Some(cell) = cells.get_mut(x) {
                *cell = Cell::Alive;
                Ok(())
            } else {
                Err(ActivateCellOutOfBoundsError::new(x, y))
            }
        } else {
            Err(ActivateCellOutOfBoundsError::new(x, y))
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::proptest;

    use super::*;

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
