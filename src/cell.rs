/// Struct representing a cell,
/// with its state
#[derive(Clone)]
pub struct Cell {
    alive: bool,
}

impl Cell {
    /// Constructor
    pub const fn new(initial_state: bool) -> Self {
        Self {
            alive: initial_state,
        }
    }
    /// Getter for the state
    pub const fn is_alive(&self) -> bool {
        self.alive
    }
    /// Setter for the state
    pub fn change_liveliness(&mut self, new_value: bool) {
        self.alive = new_value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_cell() {
        let cell = Cell::new(true);
        assert_eq!(cell.is_alive(), true);
    }

    #[test]
    fn test_default_dead_cell() {
        let cell = Cell::new(false);
        assert_eq!(cell.is_alive(), false);
    }

    #[test]
    fn test_change_liveliness() {
        let mut cell = Cell::new(true);
        assert_eq!(cell.is_alive(), true);

        cell.change_liveliness(false);
        assert_eq!(cell.is_alive(), false);

        cell.change_liveliness(true);
        assert_eq!(cell.is_alive(), true);
    }
}
