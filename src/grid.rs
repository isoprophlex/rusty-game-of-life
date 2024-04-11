use crate::cell::Cell;
#[derive(Debug, Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    //  The grid itself is a matrix
    grid: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(width: usize, height: usize, cells: Vec<Vec<Cell>>) -> Grid {
        Self {
            width,
            height,
            grid: cells
        }
    }
    pub fn update_display(&mut self) {
        let mut new_grid = self.grid.clone(); // Create a new grid to store updated cell statuses

        for pos_w in 0..self.width {
            for pos_h in 0..self.height {
                let neighbors = self.check_neighbors_for_given_pos(pos_w, pos_h);

                let live_neighbors_count = neighbors.iter().filter(|&cell| cell.is_alive()).count();

                // Apply Conway's Game of Life rules
                if self.grid[pos_h][pos_w].is_alive() {
                    if live_neighbors_count < 2 || live_neighbors_count > 3 {
                        // fewer than two live neighbors dies -> underpopulation.
                        // three live neighbors dies -> overpopulation.
                        new_grid[pos_h][pos_w].change_liveliness(false);
                    }
                } else {
                    if live_neighbors_count == 3 {
                        // live cell -> reproduction.
                        new_grid[pos_h][pos_w].change_liveliness(true);
                    }
                }
            }
        }

        // Update the grid with the new cell statuses
        self.grid = new_grid;
    }
    pub fn check_neighbors_for_given_pos(&self, g_width: usize, g_height: usize) -> Vec<Cell> {
        let mut neighbors = Vec::new();

        for dx in -1..=1 {
            for dy in -1..=1 {
                //  I'm my own cell!
                if dx == 0 && dy == 0 {
                    continue;
                }
                let x = g_width as isize + dx;
                let y = g_height as isize + dy;

                // Is in bound?
                if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
                    neighbors.push(self.grid[y as usize][x as usize].clone());
                }
            }
        }

        neighbors
    }
    pub fn grid_height(&self) -> usize {
        self.height.clone()
    }
    pub fn grid_width(&self) -> usize {
        self.width.clone()
    }
    pub fn points(&self, x: usize, y: usize) -> Cell {
        self.grid[x][y].clone()
    }
}
