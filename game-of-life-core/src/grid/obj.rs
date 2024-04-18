use super::Grid;

pub struct GameOfLifeGrid<const WIDTH: usize, const HEIGHT: usize> {
    grid: [[bool; HEIGHT]; WIDTH],
}

impl<const WIDTH: usize, const HEIGHT: usize> Grid for GameOfLifeGrid<WIDTH, HEIGHT> {
    fn new() -> Self {
        GameOfLifeGrid {
            grid: [[false; HEIGHT]; WIDTH],
        }
    }

    fn activate(&mut self, x: usize, y: usize) -> Result<(), super::ActivateCellOutOfBoundsError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_test() {

    }
}
