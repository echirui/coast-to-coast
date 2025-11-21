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
        for q in -size..=size {
            for r in (-size).max(-q - size)..=size.min(-q + size) {
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

    pub fn check_win(&self, player_color: CellState) -> bool {
        let mut visited: HashMap<Hex, bool> = HashMap::new();
        let mut queue: Vec<Hex> = Vec::new();

        // Determine starting cells based on player color
        let start_condition: Box<dyn Fn(&Hex) -> bool> = match player_color {
            CellState::Red => Box::new(|hex: &Hex| hex.q == -self.size),
            CellState::Blue => Box::new(|hex: &Hex| hex.r == -self.size),
            _ => return false, // Only Red and Blue can win
        };

        // Determine winning condition based on player color
        let win_condition: Box<dyn Fn(&Hex) -> bool> = match player_color {
            CellState::Red => Box::new(|hex: &Hex| hex.q == self.size),
            CellState::Blue => Box::new(|hex: &Hex| hex.r == self.size),
            _ => return false,
        };

        // Populate initial queue with player's pieces on the starting side
        for (hex, state) in self.cells.iter() {
            if *state == player_color && start_condition(hex) {
                queue.push(*hex);
                visited.insert(*hex, true);
            }
        }

        let mut head = 0;
        while head < queue.len() {
            let current_hex = queue[head];
            head += 1;

            // If we've reached the winning side, the player wins
            if win_condition(&current_hex) {
                return true;
            }

            // Explore neighbors
            for neighbor_hex in current_hex.get_neighbors() {
                if let Some(state) = self.get_cell(&neighbor_hex) {
                    if *state == player_color && !visited.contains_key(&neighbor_hex) {
                        visited.insert(neighbor_hex, true);
                        queue.push(neighbor_hex);
                    }
                }
            }
        }
        false
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
            let r1 = (-size).max(-q - size);
            let r2 = size.min(-q + size);
            for r in r1..=r2 {
                cell_count += 1;
                assert_eq!(
                    board.cells.get(&Hex { q, r }),
                    Some(&CellState::Empty)
                );
            }
        }
        assert_eq!(board.cells.len(), cell_count as usize);
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

    #[test]
    fn test_place_piece() {
        let mut board = Board::new(1);
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
        let mut board = Board::new(1);
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
    fn test_check_win_red_player() {
        let mut board = Board::new(1); // Small board for easy testing

        // No win initially
        assert!(!board.check_win(CellState::Red));

        // Connect a path for Red from left to right (q = -1 to q = 1)
        board.place_piece(Hex { q: -1, r: 0 }, CellState::Red).unwrap();
        board.place_piece(Hex { q: 0, r: 0 }, CellState::Red).unwrap();
        board.place_piece(Hex { q: 1, r: -1 }, CellState::Red).unwrap(); // Correct hex to connect q=1 side

        assert!(board.check_win(CellState::Red));

        // Ensure Blue doesn't win on Red's path
        assert!(!board.check_win(CellState::Blue));

        let mut board2 = Board::new(1);
        board2.place_piece(Hex { q: -1, r: 1 }, CellState::Red).unwrap();
        board2.place_piece(Hex { q: 0, r: 0 }, CellState::Red).unwrap();
        board2.place_piece(Hex { q: 1, r: -1 }, CellState::Red).unwrap();

        assert!(board2.check_win(CellState::Red));
    }

    #[test]
    fn test_check_win_blue_player() {
        let mut board = Board::new(1); // Small board for easy testing

        // No win initially
        assert!(!board.check_win(CellState::Blue));

        // Connect a path for Blue from top to bottom (r = -1 to r = 1)
        board.place_piece(Hex { q: 0, r: -1 }, CellState::Blue).unwrap();
        board.place_piece(Hex { q: 0, r: 0 }, CellState::Blue).unwrap();
        board.place_piece(Hex { q: 0, r: 1 }, CellState::Blue).unwrap();

        assert!(board.check_win(CellState::Blue));

        // Ensure Red doesn't win on Blue's path
        assert!(!board.check_win(CellState::Red));
    }

    #[test]
    fn test_check_win_no_path() {
        let mut board = Board::new(1);
        board.place_piece(Hex { q: -1, r: 0 }, CellState::Red).unwrap(); // Start of Red's side
        board.place_piece(Hex { q: 1, r: -1 }, CellState::Blue).unwrap(); // Blocking Red's path, near Red's win side
        assert!(!board.check_win(CellState::Red));
        assert!(!board.check_win(CellState::Blue));
    }
}
