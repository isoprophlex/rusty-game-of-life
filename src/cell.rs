#[derive(Debug, Clone)]
pub struct Cell {
    alive: bool,
}

impl Cell {
    pub fn new(initial_state: bool) -> Cell {
        Self {
            alive: initial_state,
        }
    }
    pub fn is_alive(&self) -> bool {
        self.alive
    }
    pub fn change_liveliness(&mut self, new_value: bool) {
        self.alive = new_value;
    }
}
