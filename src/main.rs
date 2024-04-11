use macroquad::prelude::*;

const CELL_SIZE: f32 = 10.0;
const SQUARES: i16 = 16;
#[macroquad::main("rusty-game-of-life")]
async fn main() {
    let grid_width = 80;
    let grid_height = 60;

   // let mut grid = Grid::new(grid_width, grid_height);

    loop {
        clear_background(WHITE);

        // Draw grid lines
        for x in 0..=grid_width {
            draw_line(
                x as f32 * CELL_SIZE,
                0.0,
                x as f32 * CELL_SIZE,
                grid_height as f32 * CELL_SIZE,
                1.0,
                BLACK,
            );
        }
        for y in 0..=grid_height {
            draw_line(
                0.0,
                y as f32 * CELL_SIZE,
                grid_width as f32 * CELL_SIZE,
                y as f32 * CELL_SIZE,
                1.0,
                BLACK,
            );
        }

        // Update and draw cells
       // grid.update_display();

        next_frame().await
    }
}