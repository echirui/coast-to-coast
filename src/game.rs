use crate::board::{Board, Player, PlacementError};
use std::collections::VecDeque;
use bevy::prelude::Resource;

#[derive(Debug, PartialEq)]
pub enum MoveError {
    NotYourTurn,
    Placement(PlacementError),
}

#[derive(Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Finished { winner: Player },
}

#[derive(Resource)]
pub struct Game {
    pub board: Board,
    pub current_turn: Player,
    pub state: GameState,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            current_turn: Player::White,
            state: GameState::InProgress,
        }
    }

    pub fn handle_move(&mut self, player: Player, pos1: (u32, u32), pos2: (u32, u32)) -> Result<(), MoveError> {
        if player != self.current_turn {
            return Err(MoveError::NotYourTurn);
        }

        self.board.place_block(player, pos1, pos2).map_err(MoveError::Placement)?;

        if let Some(winner) = self.check_win_condition() {
            self.state = GameState::Finished { winner };
        } else {
            self.current_turn = match self.current_turn {
                Player::White => Player::Black,
                Player::Black => Player::White,
            };
        }

        Ok(())
    }

    pub fn check_win_condition(&self) -> Option<Player> {
        let player = self.current_turn;
        let mut visited = [[false; 10]; 10];
        let mut queue = VecDeque::new();

        // Add starting cells to the queue
        for i in 0..10 {
            let pos = match player {
                Player::White => (i, 0),
                Player::Black => (0, i),
            };
            if self.board.get_cell(pos.0, pos.1).unwrap().owner == Some(player) {
                queue.push_back(pos);
                visited[pos.1 as usize][pos.0 as usize] = true;
            }
        }

        while let Some((x, y)) = queue.pop_front() {
            // Check for win condition
            match player {
                Player::White if y == 9 => return Some(Player::White),
                Player::Black if x == 9 => return Some(Player::Black),
                _ => (),
            }

            // Add neighbors to the queue
            for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx >= 0 && nx < 10 && ny >= 0 && ny < 10 {
                    let (nx, ny) = (nx as u32, ny as u32);
                    if !visited[ny as usize][nx as usize] && self.board.get_cell(nx, ny).unwrap().owner == Some(player) {
                        visited[ny as usize][nx as usize] = true;
                        queue.push_back((nx, ny));
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = Game::new();
        assert_eq!(game.current_turn, Player::White);
        assert_eq!(game.state, GameState::InProgress);
    }

    #[test]
    fn test_handle_move() {
        let mut game = Game::new();

        // Valid move
        assert_eq!(game.handle_move(Player::White, (0, 0), (1, 0)), Ok(()));
        assert_eq!(game.current_turn, Player::Black);

        // Invalid move - not your turn
        assert_eq!(game.handle_move(Player::White, (0, 1), (1, 1)), Err(MoveError::NotYourTurn));

        // Valid move for black
        assert_eq!(game.handle_move(Player::Black, (0, 1), (1, 1)), Ok(()));
        assert_eq!(game.current_turn, Player::White);

        // Invalid move - placement error
        assert_eq!(game.handle_move(Player::White, (0, 0), (0, 2)), Err(MoveError::Placement(PlacementError::NotAdjacent)));
    }

    #[test]
    fn test_check_win_condition() {
        // Test White win
        let mut game_white_wins = Game::new();
        for i in 0..10 {
            game_white_wins.board.place_block(Player::White, (0, i), (1, i)).unwrap();
        }
        assert_eq!(game_white_wins.check_win_condition(), Some(Player::White));

        // Test Black win
        let mut game_black_wins = Game::new();
        game_black_wins.current_turn = Player::Black;
        for i in 0..10 {
            game_black_wins.board.place_block(Player::Black, (i, 0), (i, 1)).unwrap();
        }
        assert_eq!(game_black_wins.check_win_condition(), Some(Player::Black));

        // Test no win
        let mut game_no_win = Game::new();
        game_no_win.board.place_block(Player::White, (0, 0), (1, 0)).unwrap();
        assert_eq!(game_no_win.check_win_condition(), None);
    }
}
