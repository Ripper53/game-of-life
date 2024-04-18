pub mod obj;

pub trait Grid {
    fn new() -> Self;
    /// Return true if cell was activated,
    /// otherwise if cell was out of bounds, return false.
    fn activate(&mut self, x: usize, y: usize) -> Result<(), ActivateCellOutOfBoundsError>;
}
#[derive(PartialEq, Eq, Debug)]
pub struct ActivateCellOutOfBoundsError {
    x: usize,
    y: usize,
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
    use proptest::prelude::*;
    
    #[test]
    fn bounds_check_test() {
        todo!();
    }
    fn bounds_check<T: Grid>() {
        let mut grid = T::new::<12, 12>();
        let activated = grid.activate(1, 1);
        assert!(activated.is_ok());
        assert_eq!((), activated.unwrap());
        let activated = grid.activate(13, 13);
        assert!(activated.is_err());
        assert_eq!(ActivateCellOutOfBoundsError { x: 13, y: 13 }, activated.unwrap_err());
        let activated = grid.activate(0, 13);
        assert!(activated.is_err());
        assert_eq!(ActivateCellOutOfBoundsError { x: 0, y: 13 }, activated.unwrap_err());
        let activated = grid.activate(13, 0);
        assert!(activated.is_err());
        assert_eq!(ActivateCellOutOfBoundsError { x: 13, y: 0 }, activated.unwrap_err());
    }
    proptest! {
        fn bound_check_random(x in 0usize..13, y in 0usize..13) {
            let mut grid = T::new::<12, 12>();
            let activated = grid.activate(x, y);
            assert!(activated.is_ok());
            assert_eq!((), activated.unwrap());
        }
    }
}
