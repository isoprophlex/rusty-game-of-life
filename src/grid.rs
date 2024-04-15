use macroquad::color::{Color};
use macroquad::prelude::draw_rectangle;
use crate::cell::Cell;
const CELL_SIZE: f32 = 10.0;
const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 100;
///  Struct containing width y height of the screen
/// and a vector of vectors of cells
pub struct Grid {
    width: usize,
    height: usize,
    alive_color: Color,
    background_color: Color,
    // The grid itself is a matrix
    grid: Vec<Vec<Cell>>,
}

impl Grid {
    /// Constructor
    pub fn new(a_color: Color, b_color: Color) -> Self {
        let mut grid = Vec::with_capacity(GRID_HEIGHT);
        for _ in 0..GRID_HEIGHT {
            let row = vec![Cell::new(false); GRID_WIDTH];
            grid.push(row);
        }
        Self {
            alive_color: a_color,
            background_color: b_color,
            width: GRID_WIDTH,
            height: GRID_HEIGHT,
            grid,
        }
    }
    /// Updates the grid checking the neighbors
    pub fn update(&mut self) {
        let mut new_grid = self.grid.clone();
        for (i, row) in new_grid.iter_mut().enumerate().take(self.height) {
            for (j, cell) in row.iter_mut().enumerate().take(self.width) {
                let live_neighbors = self.count_live_neighbors(i as isize, j as isize);
                if cell.is_alive() {
                    // Fewer than two live neighbors -> underpopulation
                    // More than three -> overpopulation.
                    if !(2..=3).contains(&live_neighbors) {
                        cell.change_liveliness(false);
                    }
                } else {
                    // Three live neighbors -> alive!
                    if live_neighbors == 3 {
                        cell.change_liveliness(true);
                    }
                }
            }
        }
        self.grid = new_grid;
    }

    /// Count live neighbors for an especific point in the grid
    fn count_live_neighbors(&self, row: isize, col: isize) -> usize {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let neighbor_row = (row + i + self.height as isize) % self.height as isize;
                let neighbor_col = (col + j + self.width as isize) % self.width as isize;
                if self.grid[neighbor_row as usize][neighbor_col as usize].is_alive() {
                    count += 1;
                }
            }
        }
        count
    }
    /// Changes the state of certain cell
    pub fn toggle_cell(&mut self, x: f32, y: f32) {
        let col = (x / CELL_SIZE) as usize;
        let row = (y / CELL_SIZE) as usize;
        if row < self.height && col < self.width {
            if let Some(cell) = self.grid.get_mut(row).and_then(|row| row.get_mut(col)) {
                cell.change_liveliness(!cell.is_alive());
            }
        }
    }

    /// Draws on screen
    pub fn draw(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                let color = if self.grid[i][j].is_alive() {
                    self.alive_color
                } else {
                    self.background_color
                };
                draw_rectangle(
                    j as f32 * CELL_SIZE,
                    i as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    color,
                );
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use macroquad::color::{BLACK, GREEN, RED, WHITE};
    use super::*;

    #[test]
    fn test_new_grid() {
        let grid = Grid::new(RED, BLACK);
        assert_eq!(grid.width, GRID_WIDTH);
        assert_eq!(grid.height, GRID_HEIGHT);
        assert_eq!(grid.grid.len(), GRID_HEIGHT);
        for row in grid.grid.iter() {
            assert_eq!(row.len(), GRID_WIDTH);
        }
    }

    #[test]
    fn test_toggle_cell() {
        let mut grid = Grid::new(BLACK, WHITE);
        grid.toggle_cell(0.0, 0.0);
        assert_eq!(grid.grid[0][0].is_alive(), true);
        grid.toggle_cell(0.0, 0.0);
        assert_eq!(grid.grid[0][0].is_alive(), false);
        grid.toggle_cell(999.0, 999.0);
    }

    #[test]
    fn test_update_grid() {
        let mut grid = Grid::new(GREEN, BLACK);
        grid.grid[0][1].change_liveliness(true);
        grid.grid[1][2].change_liveliness(true);
        grid.grid[2][0].change_liveliness(true);
        grid.grid[2][1].change_liveliness(true);
        grid.grid[2][2].change_liveliness(true);
        grid.update();
        assert_eq!(grid.grid[0][1].is_alive(), false);
        assert_eq!(grid.grid[1][2].is_alive(), true);
        assert_eq!(grid.grid[2][0].is_alive(), false);
        assert_eq!(grid.grid[2][1].is_alive(), true);
        assert_eq!(grid.grid[2][2].is_alive(), true);
    }
}
