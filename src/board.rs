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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let size = 11;
        let board = Board::new(size);

        assert_eq!(board.size, size);
        assert!(!board.cells.is_empty());

        let mut cell_count = 0;
        for q in -size..=size {
            for r in -size..=size {
                 if q + r <= size && q + r >= -size {
                    cell_count += 1;
                    assert_eq!(
                        board.cells.get(&Hex { q, r }),
                        Some(&CellState::Empty)
                    );
                }
            }
        }
        assert_eq!(board.cells.len(), cell_count);
    }

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
}
