pub mod grid;
pub mod cell;
use macroquad::prelude::*;

use crate::cell::Cell;
use crate::grid::Grid;

const CELL_SIZE: f32 = 10.0;
const SQUARES: i16 = 16;
#[macroquad::main("rusty-game-of-life")]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo run <width> <height>");
        return;
    }

    let width: usize = args[1].parse().unwrap_or(80);
    let height: usize = args[2].parse().unwrap_or(60);
    let cells= initialize_game(width, height);
   let mut grid = grid::Grid::new(width, height, cells);

    loop {
        clear_background(WHITE);

        // Draw grid lines
        for x in 0..=width {
            draw_line(
                x as f32 * CELL_SIZE,
                0.0,
                x as f32 * CELL_SIZE,
                height as f32 * CELL_SIZE,
                1.0,
                BLACK,
            );
        }
        for y in 0..=height {
            draw_line(
                0.0,
                y as f32 * CELL_SIZE,
                width as f32 * CELL_SIZE,
                y as f32 * CELL_SIZE,
                1.0,
                BLACK,
            );
        }

        // Update and draw cells
       grid.update_display();
        draw_rectangles(grid.clone());

        next_frame().await
    }
}
fn initialize_game(width: usize, height: usize) -> Vec<Vec<Cell>> {
    let mut grid = vec![vec![Cell::new(false); width]; height];

    // Set the first three elements of the third row as alive
    if height >= 3 && width >= 3 {
        grid[2][0].change_liveliness( true);
        grid[2][1].change_liveliness( true);
        grid[2][2].change_liveliness( true);
    }

    grid
}
pub fn draw_rectangles(grid: Grid) {
    for y in 0..grid.grid_height() {
        for x in 0..grid.grid_width() {
            let cell = grid.points(x, y);
            let color = if cell.is_alive() { RED } else { WHITE };
            draw_rectangle(
                x as f32 * CELL_SIZE,
                y as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                color,
            );
        }
    }
}


