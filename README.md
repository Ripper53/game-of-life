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
