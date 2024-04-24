use std::io::stdin;

use game_of_life_core::{grid::{obj::GameOfLifeGrid, Cell, Grid}, GameOfLife};

fn main() {
    let mut input = String::new();
    let mut game = GameOfLife::<GameOfLifeGrid<12, 12>>::default();
    loop {
        draw(game.grid());
        input.clear();
        stdin().read_line(&mut input);
        input = input.trim_end().to_string();
        if input.is_empty() {
            game.next();
        } else if let Some((x, y)) = input.split_once(' ') {
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            game.grid_mut().set(x, y, Cell::Alive).unwrap();
            game.grid_mut().update();
        }
    }
}

fn draw(grid: &impl Grid) {
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let cell = grid.get(x, y).unwrap();
            let cell = match cell {
                Cell::Dead => "O",
                Cell::Alive => "X",
            };
            print!("{cell}");
        }
        println!();
    }
}
