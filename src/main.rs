pub mod cell;
pub mod grid;

use std::env;
use macroquad::color::{BLACK, BLUE, Color, GREEN, RED, WHITE};
use macroquad::input::{is_key_pressed, is_mouse_button_down, KeyCode, mouse_position, MouseButton};
use macroquad::prelude::{clear_background, next_frame};
use crate::grid::Grid;


#[macroquad::main("Game of Life")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let alive_color = args.get(1).map_or(RED, |color| parse_color(color));
    let background_color = args.get(2).map_or(BLACK, |color| parse_color(color));
    let mut grid = Grid::new(alive_color, background_color);
    loop {
        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            grid.toggle_cell(mouse_x, mouse_y);
        }
        if is_key_pressed(KeyCode::Space) {
            grid.update();
        }
        clear_background(background_color);
        grid.draw();
        next_frame().await;
    }
}

fn parse_color(color: &str) -> Color {
    match color.to_lowercase().as_str() {
        "red" => RED,
        "blue" => BLUE,
        "green" => GREEN,
        "white" => WHITE,
        _ => RED,
    }
}
