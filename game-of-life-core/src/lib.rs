use grid::{Cell, Grid};

mod grid;

pub struct GameOfLife<T: Grid> {
    grid: T,
}

impl<T: Grid> Default for GameOfLife<T> {
    fn default() -> Self {
        GameOfLife { grid: T::default() }
    }
}

impl<T: Grid> GameOfLife<T> {
    pub fn next(&mut self) {
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                // We know the neighbor is in bounds
                // because we use the grid's width and height
                // for coordinates.
                // TODO: use an iterator that is provided by the grid
                // instead of manually accessing the indexes.
                let count = self.grid.neighbor_count(x, y).unwrap();
                match count {
                    0..=1 => {
                        // Death
                        self.grid.set(x, y, Cell::Dead).unwrap();
                    },
                    2 => {
                        // Stayin' Alive (or Dead)
                    },
                    3 => {
                        // Reproduction
                        self.grid.set(x, y, Cell::Alive).unwrap();
                    }
                    _ => {
                        // Overpopulation
                        self.grid.set(x, y, Cell::Dead).unwrap();
                    },
                }
            }
        }
    }
    pub fn grid(&self) -> &T {
        &self.grid
    }
    pub fn grid_mut(&mut self) -> &mut T {
        &mut self.grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::grid::obj::GameOfLifeGrid;
    use proptest::prelude::*;

    pub const WIDTH: usize = 12;
    pub const HEIGHT: usize = 12;
    pub type TestGrid = GameOfLifeGrid<WIDTH, HEIGHT>;

    #[test]
    fn next_test() {
        let mut grid = GameOfLife::<TestGrid>::default();
        grid.next();
    }
    proptest! {
        #[test]
        fn keep_grid_test(x in 0..WIDTH, y in 0..HEIGHT) {
            let mut grid = GameOfLife::<TestGrid>::default();
            let r = grid.grid.set(x, y, Cell::Alive);
            grid.grid.update();
            assert!(r.is_ok());
            let r = grid.grid().get(x, y);
            assert!(r.is_ok());
            let r = r.unwrap();
            let expected = grid.grid.get(x, y);
            assert!(expected.is_ok());
            assert_eq!(expected.unwrap(), r);
        }
        #[test]
        fn keep_grid_mut_test(x in 0..WIDTH, y in 0..HEIGHT) {
            let mut grid = GameOfLife::<TestGrid>::default();
            let r = grid.grid.set(x, y, Cell::Alive);
            grid.grid.update();
            assert!(r.is_ok());
            let r = grid.grid_mut().get(x, y);
            assert!(r.is_ok());
            let r = r.unwrap();
            let expected = grid.grid.get(x, y);
            assert!(expected.is_ok());
            assert_eq!(expected.unwrap(), r);
        }
    }
}
