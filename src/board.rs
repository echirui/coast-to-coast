use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellState {
    Empty,
    Red,
    Blue,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
}

impl Hex {
    pub fn get_neighbors(&self) -> Vec<Hex> {
        vec![
            Hex { q: self.q + 1, r: self.r },
            Hex { q: self.q, r: self.r + 1 },
            Hex { q: self.q - 1, r: self.r + 1 },
            Hex { q: self.q - 1, r: self.r },
            Hex { q: self.q, r: self.r - 1 },
            Hex { q: self.q + 1, r: self.r - 1 },
        ]
    }
}

pub struct Board {
    pub cells: HashMap<Hex, CellState>,
    pub size: i32,
}

impl Board {
    pub fn new(size: i32) -> Self {
        let mut cells = HashMap::new();
        for q in 0..size {
            for r in 0..size {
                cells.insert(Hex { q, r }, CellState::Empty);
            }
        }
        Board { cells, size }
    }

    pub fn get_cell(&self, hex: &Hex) -> Option<&CellState> {
        self.cells.get(hex)
    }

    pub fn set_cell(&mut self, hex: Hex, state: CellState) {
        self.cells.insert(hex, state);
    }

    pub fn place_piece(&mut self, hex: Hex, state: CellState) -> Result<(), &str> {
        if let Some(cell) = self.cells.get(&hex) {
            if *cell == CellState::Empty {
                self.set_cell(hex, state);
                Ok(())
            } else {
                Err("Cell is not empty")
            }
        } else {
            Err("Hex is out of bounds")
        }
    }

    pub fn is_valid_move(&self, hex: &Hex) -> bool {
        if let Some(cell) = self.cells.get(hex) {
            *cell == CellState::Empty
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set_cell() {
        let mut board = Board::new(11);
        let hex = Hex { q: 1, r: 2 };
        
        // Check initial state
        assert_eq!(board.get_cell(&hex), Some(&CellState::Empty));

        // Set and check new state
        board.set_cell(hex, CellState::Red);
        assert_eq!(board.get_cell(&hex), Some(&CellState::Red));

        // Check out of bounds
        let out_of_bounds_hex = Hex { q: 20, r: 20 };
        assert_eq!(board.get_cell(&out_of_bounds_hex), None);
    }

    #[test]
    fn test_get_neighbors() {
        let hex = Hex { q: 1, r: 2 };
        let neighbors = hex.get_neighbors();
        let expected_neighbors = vec![
            Hex { q: 2, r: 2 },
            Hex { q: 1, r: 3 },
            Hex { q: 0, r: 3 },
            Hex { q: 0, r: 2 },
            Hex { q: 1, r: 1 },
            Hex { q: 2, r: 1 },
        ];
        assert_eq!(neighbors, expected_neighbors);
    }

    #[test]
    fn test_place_piece() {
        let mut board = Board::new(2);
        let hex = Hex { q: 0, r: 0 };

        // Place a piece on an empty cell
        assert!(board.place_piece(hex, CellState::Red).is_ok());
        assert_eq!(board.get_cell(&hex), Some(&CellState::Red));

        // Try to place a piece on a non-empty cell
        assert!(board.place_piece(hex, CellState::Blue).is_err());
        assert_eq!(board.get_cell(&hex), Some(&CellState::Red)); // Should still be Red

        // Try to place a piece out of bounds
        let out_of_bounds_hex = Hex { q: 10, r: 10 };
        assert!(board.place_piece(out_of_bounds_hex, CellState::Blue).is_err());
    }

    #[test]
    fn test_is_valid_move() {
        let mut board = Board::new(2);
        let hex_empty = Hex { q: 0, r: 0 };
        let hex_occupied = Hex { q: 0, r: 1 };
        let hex_out_of_bounds = Hex { q: 10, r: 10 };

        // Empty cell
        assert!(board.is_valid_move(&hex_empty));

        // Occupied cell
        board.place_piece(hex_occupied, CellState::Red).unwrap();
        assert!(!board.is_valid_move(&hex_occupied));

        // Out of bounds cell
        assert!(!board.is_valid_move(&hex_out_of_bounds));
    }

    #[test]
    fn test_new_rhombus_board() {
        let size = 5;
        let board = Board::new(size);

        assert_eq!(board.size, size);
        assert_eq!(board.cells.len(), (size * size) as usize);

        for q in 0..size {
            for r in 0..size {
                assert!(board.cells.contains_key(&Hex { q, r }));
            }
        }

        // Check for a key that should NOT be in the map
        assert!(!board.cells.contains_key(&Hex { q: -1, r: 0 }));
        assert!(!board.cells.contains_key(&Hex { q: 0, r: -1 }));
        assert!(!board.cells.contains_key(&Hex { q: size, r: size -1 }));
        assert!(!board.cells.contains_key(&Hex { q: size -1, r: size }));
    }
}
