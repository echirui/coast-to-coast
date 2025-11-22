
use std::collections::{HashSet, VecDeque};
use crate::board::{Board, CellState, Hex};

pub const DEFAULT_BOARD_SIZE: i32 = 11;
pub const HEX_DRAW_SIZE: f32 = 20.0;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameState {
    InProgress,
    Finished { winner: CellState },
    WaitingForPieRuleChoice, // Added for pie rule
}

pub struct Game {
    pub board: Board,
    pub current_player: CellState,
    pub state: GameState,
    pub turn_count: u32, // Added to track turns for pie rule
    pub first_player_move: Option<Hex>, // Added for pie rule
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(DEFAULT_BOARD_SIZE),
            current_player: CellState::Red,
            state: GameState::InProgress,
            turn_count: 0, // Initialize turn count
            first_player_move: None, // Initialize first player move
        }
    }

    pub fn handle_click(&mut self, hex: Hex) {
        if self.state != GameState::InProgress {
            return;
        }

        if let Some(cell) = self.board.cells.get(&hex) {
            if *cell == CellState::Empty {
                self.board.set_cell(hex, self.current_player);
                self.turn_count += 1; // Increment turn count

                if self.turn_count == 1 { // After the very first move
                    self.first_player_move = Some(hex);
                    // Switch current player to the other color, as they will be the one deciding on the pie rule
                    self.current_player = match self.current_player {
                        CellState::Red => CellState::Blue,
                        CellState::Blue => CellState::Red,
                        _ => self.current_player,
                    };
                    self.state = GameState::WaitingForPieRuleChoice;
                    return; // Wait for pie rule decision
                }

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

    pub fn handle_pie_rule_decision(&mut self, apply_pie_rule: bool) {
        if self.state != GameState::WaitingForPieRuleChoice {
            return;
        }

        if apply_pie_rule {
            if let Some(first_move_hex) = self.first_player_move {
                let second_player_color = self.current_player; // The player who chose the pie rule

                // Swap the colors
                self.board.set_cell(first_move_hex, second_player_color);
                // current_player remains the same, as they now play with the swapped color.
            }
        } else {
            // No pie rule. current_player is already set to the second player after the first move,
            // so they just continue playing as that color.
        }
        self.state = GameState::InProgress; // Resume game
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
    fn test_new_game_state() {
        let game = Game::new();
        assert_eq!(game.current_player, CellState::Red);
        assert_eq!(game.state, GameState::InProgress);
        assert_eq!(game.turn_count, 0);
        assert_eq!(game.first_player_move, None);
    }

    #[test]
    fn test_first_move_triggers_pie_rule_choice() {
        let mut game = Game::new();
        let first_move_hex = Hex { q: 0, r: 0 };
        game.handle_click(first_move_hex);

        assert_eq!(game.turn_count, 1);
        assert_eq!(game.first_player_move, Some(first_move_hex));
        assert_eq!(game.state, GameState::WaitingForPieRuleChoice);
        assert_eq!(game.current_player, CellState::Blue); // Current player should be blue after first red move
        assert_eq!(game.board.get_cell(&first_move_hex), Some(&CellState::Red)); // Red placed the piece
    }

    #[test]
    fn test_pie_rule_apply() {
        let mut game = Game::new();
        let first_move_hex = Hex { q: 0, r: 0 };
        game.handle_click(first_move_hex); // Red plays 1st move

        // Game state should be WaitingForPieRuleChoice, current_player is Blue
        assert_eq!(game.state, GameState::WaitingForPieRuleChoice);
        assert_eq!(game.current_player, CellState::Blue);
        assert_eq!(game.board.get_cell(&first_move_hex), Some(&CellState::Red));

        game.handle_pie_rule_decision(true); // Blue applies pie rule

        // Board should be updated: Red's piece becomes Blue's
        assert_eq!(game.board.get_cell(&first_move_hex), Some(&CellState::Blue));
        // Game state should be InProgress
        assert_eq!(game.state, GameState::InProgress);
        // Current player should still be Blue (who chose pie rule, now playing as original Red)
        assert_eq!(game.current_player, CellState::Blue);
    }

    #[test]
    fn test_pie_rule_do_not_apply() {
        let mut game = Game::new();
        let first_move_hex = Hex { q: 0, r: 0 };
        game.handle_click(first_move_hex); // Red plays 1st move

        // Game state should be WaitingForPieRuleChoice, current_player is Blue
        assert_eq!(game.state, GameState::WaitingForPieRuleChoice);
        assert_eq!(game.current_player, CellState::Blue);
        assert_eq!(game.board.get_cell(&first_move_hex), Some(&CellState::Red));

        game.handle_pie_rule_decision(false); // Blue does not apply pie rule

        // Board should be unchanged
        assert_eq!(game.board.get_cell(&first_move_hex), Some(&CellState::Red));
        // Game state should be InProgress
        assert_eq!(game.state, GameState::InProgress);
        // Current player should remain Blue
        assert_eq!(game.current_player, CellState::Blue);
    }

    #[test]
    fn test_subsequent_moves_after_pie_rule_decision() {
        let mut game = Game::new();
        let first_move_hex = Hex { q: 0, r: 0 };
        game.handle_click(first_move_hex); // Red plays 1st move
        game.handle_pie_rule_decision(true); // Blue applies pie rule, Red's piece is now Blue's, Blue plays as Red.

        // Blue's turn (as Red color)
        assert_eq!(game.current_player, CellState::Blue);
        let second_move_hex = Hex { q: 1, r: 0 };
        game.handle_click(second_move_hex);

        assert_eq!(game.board.get_cell(&second_move_hex), Some(&CellState::Blue));
        assert_eq!(game.current_player, CellState::Red); // Red's turn (as Blue color)
        assert_eq!(game.turn_count, 2);
    }

    #[test]
    fn test_subsequent_moves_after_no_pie_rule_decision() {
        let mut game = Game::new();
        let first_move_hex = Hex { q: 0, r: 0 };
        game.handle_click(first_move_hex); // Red plays 1st move
        game.handle_pie_rule_decision(false); // Blue does not apply pie rule, Blue plays as Blue.

        // Blue's turn (as Blue color)
        assert_eq!(game.current_player, CellState::Blue);
        let second_move_hex = Hex { q: 1, r: 0 };
        game.handle_click(second_move_hex);

        assert_eq!(game.board.get_cell(&second_move_hex), Some(&CellState::Blue));
        assert_eq!(game.current_player, CellState::Red); // Red's turn (as Red color)
        assert_eq!(game.turn_count, 2);
    }
}
