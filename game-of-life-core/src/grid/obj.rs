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
    use super::*;

    #[test]
    fn activation_test() {
        let mut grid = GameOfLifeGrid::<4, 4>::default();
        let r = grid.grid[1][1];
        assert_eq!(Cell::Dead, r);
        let r = grid.activate(1, 1);
        assert!(r.is_ok());
        let r = grid.grid[1][1];
        assert_eq!(Cell::Alive, r);
    }
}
