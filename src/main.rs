mod board;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::board::{Board, Block, Orientation};

    #[test]
    fn it_creates_a_10x10_board() {
        let board = Board::new();
        assert_eq!(board.size(), (10, 10));
    }

    #[test]
    fn it_places_a_block() {
        let mut board = Board::new();
        let block = Block::new(Orientation::Horizontal);
        board.place_block(block, 0, 0);
        assert_eq!(board.get(0, 0), 1);
        assert_eq!(board.get(0, 1), 1);
    }

    #[test]
    fn it_places_a_vertical_block() {
        let mut board = Board::new();
        let block = Block::new(Orientation::Vertical);
        board.place_block(block, 0, 0);
        assert_eq!(board.get(0, 0), 1);
        assert_eq!(board.get(1, 0), 1);
    }
}
