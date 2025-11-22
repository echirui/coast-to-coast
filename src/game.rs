
use std::collections::{HashSet, VecDeque};
use crate::board::{Board, CellState, Hex};

const DEFAULT_BOARD_SIZE: i32 = 11;
pub const HEX_DRAW_SIZE: f32 = 20.0;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameState {
    InProgress,
    Finished { winner: CellState },
}

pub struct Game {
    pub board: Board,
    pub current_player: CellState,
    pub state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(DEFAULT_BOARD_SIZE),
            current_player: CellState::Red,
            state: GameState::InProgress,
        }
    }

    pub fn handle_click(&mut self, hex: Hex) {
        if self.state != GameState::InProgress {
            return;
        }

        if let Some(cell) = self.board.cells.get(&hex) {
            if *cell == CellState::Empty {
                self.board.set_cell(hex, self.current_player);
                if self.check_win_condition() {
                    self.state = GameState::Finished { winner: self.current_player };
                } else {
                    self.current_player = match self.current_player {
                        CellState::Red => CellState::Blue,
                        CellState::Blue => CellState::Red,
                        _ => self.current_player,
                    };
                }
            }
        }
    }

    fn check_win_condition(&self) -> bool {
        let size = self.board.size;
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        let (start_condition, end_condition): (Box<dyn Fn(Hex) -> bool>, Box<dyn Fn(Hex) -> bool>) = match self.current_player {
            CellState::Red => (
                Box::new(move |h: Hex| h.q == 0),
                Box::new(move |h: Hex| h.q == size - 1),
            ),
            CellState::Blue => (
                Box::new(move |h: Hex| h.r == 0),
                Box::new(move |h: Hex| h.r == size - 1),
            ),
            _ => return false,
        };

        for (hex, state) in &self.board.cells {
            if *state == self.current_player && start_condition(*hex) {
                queue.push_back(*hex);
                visited.insert(*hex);
            }
        }

        while let Some(hex) = queue.pop_front() {
            if end_condition(hex) {
                return true;
            }

            for neighbor in hex.get_neighbors() {
                if !visited.contains(&neighbor) {
                    if let Some(state) = self.board.cells.get(&neighbor) {
                        if *state == self.current_player {
                            visited.insert(neighbor);
                            queue.push_back(neighbor);
                        }
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
    fn test_red_wins() {
        let mut game = Game::new();
        game.board = Board::new(3); // Use a smaller board for testing
        game.current_player = CellState::Red;

        // Create a winning path for Red player (q=0 to q=2)
        game.board.set_cell(Hex { q: 0, r: 1 }, CellState::Red);
        game.board.set_cell(Hex { q: 1, r: 1 }, CellState::Red);
        game.board.set_cell(Hex { q: 2, r: 1 }, CellState::Red);
        
        assert!(game.check_win_condition());
    }

    #[test]
    fn test_blue_wins() {
        let mut game = Game::new();
        game.board = Board::new(3);
        game.current_player = CellState::Blue;

        // Create a winning path for Blue player (r=0 to r=2)
        game.board.set_cell(Hex { q: 1, r: 0 }, CellState::Blue);
        game.board.set_cell(Hex { q: 1, r: 1 }, CellState::Blue);
        game.board.set_cell(Hex { q: 1, r: 2 }, CellState::Blue);

        assert!(game.check_win_condition());
    }

    #[test]
    fn test_no_win() {
        let mut game = Game::new();
        game.board = Board::new(3);

        // Red's turn, but no winning path
        game.current_player = CellState::Red;
        game.board.set_cell(Hex { q: 0, r: 1 }, CellState::Red);
        game.board.set_cell(Hex { q: 2, r: 1 }, CellState::Red);
        assert!(!game.check_win_condition());

        // Blue's turn, but no winning path
        game.current_player = CellState::Blue;
        game.board.set_cell(Hex { q: 1, r: 0 }, CellState::Blue);
        game.board.set_cell(Hex { q: 1, r: 2 }, CellState::Blue);
        assert!(!game.check_win_condition());
    }

    #[test]
    fn test_red_wins_zigzag() {
        let mut game = Game::new();
        game.board = Board::new(4);
        game.current_player = CellState::Red;

        // Create a zigzag winning path for Red player (q=0 to q=3)
        game.board.set_cell(Hex { q: 0, r: 1 }, CellState::Red);
        game.board.set_cell(Hex { q: 1, r: 0 }, CellState::Red);
        game.board.set_cell(Hex { q: 1, r: 1 }, CellState::Red);
        game.board.set_cell(Hex { q: 2, r: 1 }, CellState::Red);
        game.board.set_cell(Hex { q: 3, r: 0 }, CellState::Red);

        assert!(game.check_win_condition());
    }

    #[test]
    fn test_blue_wins_zigzag() {
        let mut game = Game::new();
        game.board = Board::new(4);
        game.current_player = CellState::Blue;

        // Create a zigzag winning path for Blue player (r=0 to r=3)
        game.board.set_cell(Hex { q: 1, r: 0 }, CellState::Blue);
        game.board.set_cell(Hex { q: 0, r: 1 }, CellState::Blue);
        game.board.set_cell(Hex { q: 1, r: 1 }, CellState::Blue);
        game.board.set_cell(Hex { q: 1, r: 2 }, CellState::Blue);
        game.board.set_cell(Hex { q: 0, r: 3 }, CellState::Blue);

        assert!(game.check_win_condition());
    }
}
