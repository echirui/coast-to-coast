#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    White,
    Black,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cell {
    pub owner: Option<Player>,
    pub height: u32,
}

impl Cell {
    fn new() -> Self {
        Self {
            owner: None,
            height: 0,
        }
    }
}

pub struct Board {
    pub grid: [[Cell; 10]; 10],
}

#[derive(Debug, PartialEq)]
pub enum PlacementError {
    OutOfBounds,
    Occupied,
    DifferentHeights,
    NotAdjacent,
}


impl Board {
    pub fn new() -> Board {
        Board {
            grid: [[Cell::new(); 10]; 10],
        }
    }

    pub fn get_cell(&self, x: u32, y: u32) -> Option<&Cell> {
        if x < 10 && y < 10 {
            Some(&self.grid[y as usize][x as usize])
        } else {
            None
        }
    }

    pub fn place_block(&mut self, player: Player, pos1: (u32, u32), pos2: (u32, u32)) -> Result<(), PlacementError> {
        // 1. Check for adjacency
        let (x1, y1) = pos1;
        let (x2, y2) = pos2;
        if (x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs() != 1 {
            return Err(PlacementError::NotAdjacent);
        }

        // 2. Check for out of bounds
        if x1 >= 10 || y1 >= 10 || x2 >= 10 || y2 >= 10 {
            return Err(PlacementError::OutOfBounds);
        }

        // 3. Check heights and ownership
        {
            let cell1 = &self.grid[y1 as usize][x1 as usize];
            let cell2 = &self.grid[y2 as usize][x2 as usize];

            if cell1.height != cell2.height {
                return Err(PlacementError::DifferentHeights);
            }

            if cell1.owner.is_some() && cell1.owner != Some(player) {
                return Err(PlacementError::Occupied);
            }

            if cell2.owner.is_some() && cell2.owner != Some(player) {
                return Err(PlacementError::Occupied);
            }
        }

        // 4. Place the block
        let cell1 = &mut self.grid[y1 as usize][x1 as usize];
        cell1.owner = Some(player);
        cell1.height += 1;

        let cell2 = &mut self.grid[y2 as usize][x2 as usize];
        cell2.owner = Some(player);
        cell2.height += 1;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let board = Board::new();
        for row in board.grid.iter() {
            for cell in row.iter() {
                assert_eq!(cell.owner, None);
                assert_eq!(cell.height, 0);
            }
        }
    }

    #[test]
    fn test_get_cell() {
        let board = Board::new();
        // Test inside the board
        assert!(board.get_cell(0, 0).is_some());
        assert!(board.get_cell(9, 9).is_some());

        // Test outside the board
        assert!(board.get_cell(10, 0).is_none());
        assert!(board.get_cell(0, 10).is_none());
        assert!(board.get_cell(10, 10).is_none());
    }

    #[test]
    fn test_place_block() {
        let mut board = Board::new();

        // Successful horizontal placement
        assert_eq!(board.place_block(Player::White, (0, 0), (1, 0)), Ok(()));
        assert_eq!(board.get_cell(0, 0).unwrap().owner, Some(Player::White));
        assert_eq!(board.get_cell(1, 0).unwrap().owner, Some(Player::White));
        assert_eq!(board.get_cell(0, 0).unwrap().height, 1);
        assert_eq!(board.get_cell(1, 0).unwrap().height, 1);

        // Successful vertical placement
        assert_eq!(board.place_block(Player::Black, (0, 1), (0, 2)), Ok(()));
        assert_eq!(board.get_cell(0, 1).unwrap().owner, Some(Player::Black));
        assert_eq!(board.get_cell(0, 2).unwrap().owner, Some(Player::Black));
        assert_eq!(board.get_cell(0, 1).unwrap().height, 1);
        assert_eq!(board.get_cell(0, 2).unwrap().height, 1);

        // Successful stacking
        assert_eq!(board.place_block(Player::White, (0, 0), (1, 0)), Ok(()));
        assert_eq!(board.get_cell(0, 0).unwrap().height, 2);
        assert_eq!(board.get_cell(1, 0).unwrap().height, 2);

        // Error: Not adjacent
        assert_eq!(board.place_block(Player::White, (0, 0), (2, 0)), Err(PlacementError::NotAdjacent));

        // Error: Out of bounds
        assert_eq!(board.place_block(Player::White, (9, 0), (10, 0)), Err(PlacementError::OutOfBounds));

        // Error: Different heights
        assert_eq!(board.place_block(Player::White, (0, 0), (0, 1)), Err(PlacementError::DifferentHeights));

        // Error: Occupied by another player
        assert_eq!(board.place_block(Player::White, (0, 1), (0, 2)), Err(PlacementError::Occupied));
    }
}