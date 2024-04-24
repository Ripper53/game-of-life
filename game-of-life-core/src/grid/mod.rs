pub mod obj;

pub trait Grid: Default {
    /// Return Ok with a copy of the cell's value if in bounds,
    /// otherwise if cell was out of bounds, return Err.
    fn get(&self, x: usize, y: usize) -> Result<Cell, CellOutOfBoundsError>;
    /// Return Ok if cell was activated (state became `Cell::Alive`),
    /// otherwise if cell was out of bounds, return Err.
    fn activate(&mut self, x: usize, y: usize) -> Result<(), CellOutOfBoundsError>;
    /// Returns the number of alive neighbors the cell has,
    /// if out of bounds, returns Err.
    fn neighbor_count(&self, x: usize, y: usize) -> Result<usize, CellOutOfBoundsError> {
        match self.get(x, y) {
            Ok(_) => {
                Ok([
                    self.get(x, y + 1),
                    self.get(x + 1, y + 1),
                    self.get(x + 1, y),
                    self.get(x + 1, y - 1),
                    self.get(x, y - 1),
                    self.get(x - 1, y - 1),
                    self.get(x - 1, y),
                    self.get(x - 1, y + 1),
                ]
                .into_iter()
                .filter(|r| matches!(r, Ok(Cell::Alive)))
                .count())
            },
            Err(e) => Err(e),
        }
    }
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

// No default,
// when a grid is created,
// it should decide how it wants
// each of its cells' states to be.
// Tests are written to assure the initial
// state of such grids matches what is desired.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Cell {
    Dead,
    Alive,
}

#[derive(PartialEq, Eq, Debug)]
pub struct CellOutOfBoundsError {
    // Fields are private
    // because we want to only
    // allow read access so
    // the user knows what
    // coordinates caused the error.
    // Avoids unintentional mutation.
    x: usize,
    y: usize,
}

impl CellOutOfBoundsError {
    pub(crate) fn new(x: usize, y: usize) -> Self {
        // New function stays within crate because we only want
        // error to be instantiated by the game.
        CellOutOfBoundsError { x, y }
    }
    pub fn x(&self) -> usize { self.x }
    pub fn y(&self) -> usize { self.y }
}
impl std::error::Error for CellOutOfBoundsError {}
impl std::fmt::Display for CellOutOfBoundsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cell ({}, {}) is out of bounds of the grid", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use obj::*;
    use proptest::prelude::*;
    use crate::tests::*;

    /// Tests activating a cell within the grid works,
    /// and activating a cell out of bounds returns an error.
    #[test]
    fn activate_bounds_check_test() {
        let mut grid = TestGrid::default();
        let activated = grid.activate(1, 1);
        assert!(activated.is_ok());
        assert_eq!((), activated.unwrap());
        const OUT_OF_BOUNDS_WIDTH: usize = WIDTH + 1;
        const OUT_OF_BOUNDS_HEIGHT: usize = HEIGHT + 1;
        let activated = grid.activate(OUT_OF_BOUNDS_WIDTH, OUT_OF_BOUNDS_HEIGHT);
        assert!(activated.is_err());
        assert_eq!(CellOutOfBoundsError::new(OUT_OF_BOUNDS_WIDTH, OUT_OF_BOUNDS_HEIGHT), activated.unwrap_err());
        let activated = grid.activate(0, OUT_OF_BOUNDS_HEIGHT);
        assert!(activated.is_err());
        assert_eq!(CellOutOfBoundsError::new(0, OUT_OF_BOUNDS_HEIGHT), activated.unwrap_err());
        let activated = grid.activate(OUT_OF_BOUNDS_WIDTH, 0);
        assert!(activated.is_err());
        assert_eq!(CellOutOfBoundsError::new(OUT_OF_BOUNDS_WIDTH, 0), activated.unwrap_err());
    }
    #[test]
    fn no_neighbor_count_test() {
        let mut grid = TestGrid::default();
        let r = grid.neighbor_count(1, 1);
        assert!(r.is_ok());
        assert_eq!(0, r.unwrap());
        let r = grid.activate(1, 1);
        assert!(r.is_ok());
        let r = grid.neighbor_count(1, 1);
        assert!(r.is_ok());
        assert_eq!(0, r.unwrap());
    }
    #[test]
    fn out_of_bounds_neightbor_count_test() {
        let mut grid = TestGrid::default();
        let r = grid.neighbor_count(WIDTH, HEIGHT);
        assert!(r.is_err());
        let r = grid.activate(WIDTH - 1, HEIGHT - 1);
        assert!(r.is_ok());
        let r = grid.neighbor_count(WIDTH, HEIGHT);
        assert!(r.is_err());
    }
    #[test]
    fn neighbor_count_test() {
        const POS_X: usize = 1;
        const POS_Y: usize = 1;
        let mut grid = TestGrid::default();

        // Activate Neighbors
        let r = grid.activate(POS_X, POS_Y + 1);
        assert!(r.is_ok());
        let r = grid.neighbor_count(1, 1);
        assert!(r.is_ok());
        assert_eq!(1, r.unwrap());

        let r = grid.activate(POS_X + 1, POS_Y + 1);
        assert!(r.is_ok());
        let r = grid.neighbor_count(1, 1);
        assert!(r.is_ok());
        assert_eq!(2, r.unwrap());

        let r = grid.activate(POS_X + 1, POS_Y);
        assert!(r.is_ok());
        let r = grid.neighbor_count(1, 1);
        assert!(r.is_ok());
        assert_eq!(3, r.unwrap());

        let r = grid.activate(POS_X + 1, POS_Y - 1);
        assert!(r.is_ok());
        let r = grid.neighbor_count(1, 1);
        assert!(r.is_ok());
        assert_eq!(4, r.unwrap());

        let r = grid.activate(POS_X, POS_Y - 1);
        assert!(r.is_ok());
        let r = grid.neighbor_count(1, 1);
        assert!(r.is_ok());
        assert_eq!(5, r.unwrap());

        let r = grid.activate(POS_X - 1, POS_Y - 1);
        assert!(r.is_ok());
        let r = grid.neighbor_count(1, 1);
        assert!(r.is_ok());
        assert_eq!(6, r.unwrap());

        let r = grid.activate(POS_X - 1, POS_Y);
        assert!(r.is_ok());
        let r = grid.neighbor_count(1, 1);
        assert!(r.is_ok());
        assert_eq!(7, r.unwrap());

        let r = grid.activate(POS_X - 1, POS_Y + 1);
        assert!(r.is_ok());
        let r = grid.neighbor_count(1, 1);
        assert!(r.is_ok());
        assert_eq!(8, r.unwrap());

        // The cell itself, not its neighbor.
        let r = grid.activate(POS_X, POS_Y);
        assert!(r.is_ok());
        let r = grid.neighbor_count(1, 1);
        assert!(r.is_ok());
        assert_eq!(8, r.unwrap());

        // Not a neighbor.
        let r = grid.activate(POS_X + 2, POS_Y);
        assert!(r.is_ok());
        let r = grid.neighbor_count(1, 1);
        assert!(r.is_ok());
        assert_eq!(8, r.unwrap());
    }
    proptest! {
        /// Tests the constructor of the `CellOutOfBoundsError` type.
        #[test]
        fn new_cell_of_of_bounds_error_test(x in 0usize..usize::MAX, y in 0usize..usize::MAX) {
            let e = CellOutOfBoundsError::new(x, y);
            assert_eq!(CellOutOfBoundsError { x: x, y: y }, e);
            assert_eq!(x, e.x());
            assert_eq!(y, e.y());
        }
        /// Tests if the correct text is displayed for the error.
        #[test]
        fn display_error_test(x in 0usize..usize::MAX, y in 0usize..usize::MAX) {
            let e = CellOutOfBoundsError::new(x, y);
            assert_eq!(format!("cell ({}, {}) is out of bounds of the grid", x, y), e.to_string());
        }
        /// Tests if retrieving a cell within the bounds of the grid works.
        #[test]
        fn get_in_bound_check_random_test(x in 0usize..WIDTH, y in 0usize..HEIGHT) {
            let mut grid = TestGrid::default();
            let activated = grid.get(x, y);
            assert!(activated.is_ok());
            assert_eq!(Cell::Dead, activated.unwrap());
        }
        /// Tests if retrieving a cell outside of the bounds of the grid returns an error.
        #[test]
        fn get_out_bound_check_random_test(x in WIDTH..usize::MAX, y in HEIGHT..usize::MAX) {
            let mut grid = TestGrid::default();
            let activated = grid.get(x, y);
            assert!(activated.is_err());
            assert_eq!(CellOutOfBoundsError::new(x, y), activated.unwrap_err());
        }
        /// Tests if activating a cell within the bounds of the grid works.
        #[test]
        fn activate_in_bound_check_random_test(x in 0usize..WIDTH, y in 0usize..HEIGHT) {
            let mut grid = TestGrid::default();
            let activated = grid.activate(x, y);
            assert!(activated.is_ok());
            assert_eq!((), activated.unwrap());
        }
        /// Tests if activating a cell outside of the bounds of the grid returns an error.
        #[test]
        fn activate_out_bound_check_random_test(x in WIDTH..usize::MAX, y in HEIGHT..usize::MAX) {
            let mut grid = TestGrid::default();
            let activated = grid.activate(x, y);
            assert!(activated.is_err());
            assert_eq!(CellOutOfBoundsError::new(x, y), activated.unwrap_err());
        }
    }
}
