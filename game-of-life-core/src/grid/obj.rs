use super::Grid;

pub struct GameOfLifeGrid<const WIDTH: usize, const HEIGHT: usize> {
    grid: [[Cell; HEIGHT]; WIDTH],
}

#[derive(Clone, Copy)]
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
    fn activate(&mut self, x: usize, y: usize) -> Result<(), super::ActivateCellOutOfBoundsError> {
        if let Some(cells) = self.grid.get_mut(y) {
            if let Some(cell) = cells.get_mut(x) {
                // TODO
                Ok(())
            } else {
                Err(super::ActivateCellOutOfBoundsError::new(x, y))
            }
        } else {
            Err(super::ActivateCellOutOfBoundsError::new(x, y))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_test() {

    }
}
