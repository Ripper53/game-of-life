use grid::{Cell, Grid};

pub mod grid;

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
        self.grid.update();
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

    proptest! {
        /// Tests death of cell via underpopulation,
        /// because it is alone.
        #[test]
        fn next_alone_death_test(x in 0..WIDTH, y in 0..HEIGHT) {
            let mut game = GameOfLife::<TestGrid>::default();
            game.grid_mut().set(x, y, Cell::Alive);
            game.grid_mut().update();
            let cell = game.grid().get(x, y);
            assert!(cell.is_ok());
            assert_eq!(Cell::Alive, cell.unwrap());
            game.next();
            let cell = game.grid().get(x, y);
            assert!(cell.is_ok());
            assert_eq!(Cell::Dead, cell.unwrap());
        }
        /// Tests death of cell via underpopulation,
        /// but not alone.
        #[test]
        fn next_underpopulation_death_test(x in 0..WIDTH-1, y in 0..HEIGHT) {
            let mut game = GameOfLife::<TestGrid>::default();
            game.grid_mut().set(x, y, Cell::Alive);
            game.grid_mut().set(x + 1, y, Cell::Alive);
            game.grid_mut().update();
            let cell = game.grid().get(x, y);
            assert!(cell.is_ok());
            assert_eq!(Cell::Alive, cell.unwrap());
            let cell = game.grid().get(x + 1, y);
            assert!(cell.is_ok());
            assert_eq!(Cell::Alive, cell.unwrap());
            game.next();
            let cell = game.grid().get(x, y);
            assert!(cell.is_ok());
            assert_eq!(Cell::Dead, cell.unwrap());
            let cell = game.grid().get(x + 1, y);
            assert!(cell.is_ok());
            assert_eq!(Cell::Dead, cell.unwrap());
        }
        /// Tests death of cell via overpopulation.
        #[test]
        fn next_overpopulation_death_test(x in 1..WIDTH-1, y in 0..HEIGHT-1) {
            let mut game = GameOfLife::<TestGrid>::default();
            game.grid_mut().set(x, y, Cell::Alive);
            game.grid_mut().set(x + 1, y, Cell::Alive);
            game.grid_mut().set(x + 1, y + 1, Cell::Alive);
            game.grid_mut().set(x, y + 1, Cell::Alive);
            game.grid_mut().set(x - 1, y + 1, Cell::Alive);
            game.grid_mut().update();
            let cell = game.grid().get(x, y);
            assert!(cell.is_ok());
            assert_eq!(Cell::Alive, cell.unwrap());
            game.next();
            let cell = game.grid().get(x, y);
            assert!(cell.is_ok());
            assert_eq!(Cell::Dead, cell.unwrap());
        }
        /// Tests production of a cell, and the staying alive of cells.
        #[test]
        fn next_produce_square_pattern_test(x in 1..WIDTH-1, y in 1..HEIGHT-1) {
            let mut game = GameOfLife::<TestGrid>::default();
            // Keep cell at (1, 1) alive
            game.grid_mut().set(x, y, Cell::Alive);
            game.grid_mut().set(x + 1, y, Cell::Alive);
            game.grid_mut().set(x, y + 1, Cell::Alive);
            game.grid_mut().update();
            // Run behavior a few times
            // state should not change after
            // first call of `next`
            for _ in 0..4 {
                // Next should keep all cells alive,
                // AND make a fourth one at (2, 2)
                game.next();
                // User Activated Cells
                let cell = game.grid().get(x, y);
                assert!(cell.is_ok());
                assert_eq!(Cell::Alive, cell.unwrap());
                let cell = game.grid().get(x + 1, y);
                assert!(cell.is_ok());
                assert_eq!(Cell::Alive, cell.unwrap());
                let cell = game.grid().get(x, y + 1);
                assert!(cell.is_ok());
                assert_eq!(Cell::Alive, cell.unwrap());
                // Fourth Automatic Activated Cell
                // because we called `next`
                let cell = game.grid().get(x + 1, y + 1);
                assert!(cell.is_ok());
                assert_eq!(Cell::Alive, cell.unwrap());
            }
        }
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
