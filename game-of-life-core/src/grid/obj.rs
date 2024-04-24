use super::{Cell, CellOutOfBoundsError, Grid};

pub struct GameOfLifeGrid<const WIDTH: usize, const HEIGHT: usize> {
    grid: [[CellHolder; HEIGHT]; WIDTH],
}

#[derive(Clone, Copy, Debug)]
struct CellHolder {
    cell: Cell,
    change_to: Option<Cell>,
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for GameOfLifeGrid<WIDTH, HEIGHT> {
    fn default() -> Self {
        GameOfLifeGrid {
            grid: [[CellHolder { cell: Cell::Dead, change_to: None }; HEIGHT]; WIDTH],
        }
    }
}
impl<const WIDTH: usize, const HEIGHT: usize> Grid for GameOfLifeGrid<WIDTH, HEIGHT> {
    fn get(&self, x: usize, y: usize) -> Result<Cell, CellOutOfBoundsError> {
        if let Some(cells) = self.grid.get(y) {
            if let Some(cell) = cells.get(x) {
                Ok(cell.cell)
            } else {
                Err(CellOutOfBoundsError::new(x, y))
            }
        } else {
            Err(CellOutOfBoundsError::new(x, y))
        }
    }
    fn set(&mut self, x: usize, y: usize, cell: Cell) -> Result<(), CellOutOfBoundsError> {
        if let Some(cells) = self.grid.get_mut(y) {
            if let Some(cell_to_change) = cells.get_mut(x) {
                cell_to_change.change_to = Some(cell);
                Ok(())
            } else {
                Err(CellOutOfBoundsError::new(x, y))
            }
        } else {
            Err(CellOutOfBoundsError::new(x, y))
        }
    }
    fn update(&mut self) {
        for cell in self.grid.iter_mut().flatten() {
            if let Some(updated_cell) = cell.change_to.take() {
                cell.cell = updated_cell;
            }
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
                let direct_r = grid.grid[y][x].cell;
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
            let r = grid.grid[y][x].cell;
            assert_eq!(Cell::Dead, r);
            let r = grid.set(x, y, Cell::Alive);
            assert!(r.is_ok());
            let r = grid.grid[y][x].change_to.unwrap();
            assert_eq!(Cell::Alive, r);
        }
    }
}
