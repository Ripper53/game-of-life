pub mod obj;

pub trait Grid: Default {
    /// Return true if cell was activated,
    /// otherwise if cell was out of bounds, return false.
    fn activate(&mut self, x: usize, y: usize) -> Result<(), ActivateCellOutOfBoundsError>;
}
#[derive(PartialEq, Eq, Debug)]
pub struct ActivateCellOutOfBoundsError {
    x: usize,
    y: usize,
}
impl ActivateCellOutOfBoundsError {
    pub(crate) fn new(x: usize, y: usize) -> Self {
        ActivateCellOutOfBoundsError { x, y }
    }
    pub fn x(&self) -> usize { self.x }
    pub fn y(&self) -> usize { self.y }
}
impl std::error::Error for ActivateCellOutOfBoundsError {}
impl std::fmt::Display for ActivateCellOutOfBoundsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cell ({}, {}) is out of bounds of the grid", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use obj::*;
    use proptest::prelude::*;

    const WIDTH: usize = 12;
    const HEIGHT: usize = 12;
    type TestGrid = GameOfLifeGrid<WIDTH, HEIGHT>;
    #[test]
    fn bounds_check_test() {
        let mut grid = TestGrid::default();
        let activated = grid.activate(1, 1);
        assert!(activated.is_ok());
        assert_eq!((), activated.unwrap());
        const OUT_OF_BOUNDS_WIDTH: usize = WIDTH + 1;
        const OUT_OF_BOUNDS_HEIGHT: usize = HEIGHT + 1;
        let activated = grid.activate(OUT_OF_BOUNDS_WIDTH, OUT_OF_BOUNDS_HEIGHT);
        assert!(activated.is_err());
        assert_eq!(ActivateCellOutOfBoundsError::new(OUT_OF_BOUNDS_WIDTH, OUT_OF_BOUNDS_HEIGHT), activated.unwrap_err());
        let activated = grid.activate(0, OUT_OF_BOUNDS_HEIGHT);
        assert!(activated.is_err());
        assert_eq!(ActivateCellOutOfBoundsError::new(0, OUT_OF_BOUNDS_HEIGHT), activated.unwrap_err());
        let activated = grid.activate(OUT_OF_BOUNDS_WIDTH, 0);
        assert!(activated.is_err());
        assert_eq!(ActivateCellOutOfBoundsError::new(OUT_OF_BOUNDS_WIDTH, 0), activated.unwrap_err());
    }
    proptest! {
        #[test]
        fn new_activate_cell_of_of_bounds_error(x in 0usize..usize::MAX, y in 0usize..usize::MAX) {
            let e = ActivateCellOutOfBoundsError::new(x, y);
            assert_eq!(ActivateCellOutOfBoundsError { x: x, y: y }, e);
            assert_eq!(x, e.x());
            assert_eq!(y, e.y());
        }
        #[test]
        fn display_error_test(x in 0usize..usize::MAX, y in 0usize..usize::MAX) {
            let e = ActivateCellOutOfBoundsError::new(x, y);
            assert_eq!(format!("cell ({}, {}) is out of bounds of the grid", x, y), e.to_string());
        }
        #[test]
        fn in_bound_check_random(x in 0usize..WIDTH, y in 0usize..HEIGHT) {
            let mut grid = TestGrid::default();
            let activated = grid.activate(x, y);
            assert!(activated.is_ok());
            assert_eq!((), activated.unwrap());
        }
        #[test]
        fn out_bound_check_random(x in WIDTH..usize::MAX, y in HEIGHT..usize::MAX) {
            let mut grid = TestGrid::default();
            let activated = grid.activate(x, y);
            assert!(activated.is_err());
            assert_eq!(ActivateCellOutOfBoundsError::new(x, y), activated.unwrap_err());
        }
    }
}
