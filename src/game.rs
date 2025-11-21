
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
