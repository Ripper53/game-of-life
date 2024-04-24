# Game of Life

Conway's game of life written in Rust with tests.
This project was to help me understand how to test code, and use the tools listed below to create better tests.

## Tests
Tests are written for the `game-of-life-core` project.
Using [`proptest`](https://github.com/proptest-rs/proptest) for chaos testing.
Using [`cargo-mutants`](https://github.com/sourcefrog/cargo-mutants) for logic testing.

Simply run `cargo test` to run the tests, and `cargo mutants` to run mutant tests.
If `cargo mutants` does not work, you may need to install it first using the following command: `cargo install --locked cargo-mutants`

## Run
Go into the `game-of-life-terminal` directory, and run `cargo run`.
There, you can test the game. You can enter input such as `1 2` (space between the numbers) to activate a cell at the position `(1, 2)`.
You can keep entering coordinates until you want a new generation to happen.
Simply press `Enter` without any input to have the game move into the next generation.

## Code
You can generate a 12x12 grid with `GameOfLife::<GameOfLifeGrid<12, 12>>::default()`.

The grid of the game is different from the game itself. You can have a grid, without the game.
The game is the generation logic, while the grid is simply the grid logic.
The grid turns cells on and off, while the game tells the grid which cells to turn on and off.

You can implement your own grid as well, using the `Grid` trait. The grid which is already provided (`GameOfLifeGrid`), stores its current cell value, and its value that will be changed during the next generation, in a two-dimensional array. First index represent the row, whereas the second represents the column. Ex. `grid[y][x]`

You could have a different implementation, such as storing the coordinates that need to be updated along with their values in a vector instead, or using a one-dimensional array. Regardless, the game does not care about the implementation of the grid.

NOTE: you can change the grid used for tests to any you implement by changing the type alias `TestGrid` under `game-of-life-core/src/lib.rs` so you can test your own implementation.
