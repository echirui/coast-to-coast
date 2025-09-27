pub enum Orientation {
    Horizontal,
    Vertical,
}

pub struct Block {
    pub orientation: Orientation,
}

impl Block {
    pub fn new(orientation: Orientation) -> Block {
        Block { orientation }
    }
}

pub struct Board {
    grid: Vec<Vec<i32>>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            grid: vec![vec![0; 10]; 10],
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.grid.len(), self.grid[0].len())
    }

    pub fn place_block(&mut self, block: Block, x: usize, y: usize) {
        match block.orientation {
            Orientation::Horizontal => {
                self.grid[x][y] = 1;
                self.grid[x][y + 1] = 1;
            }
            Orientation::Vertical => {
                self.grid[x][y] = 1;
                self.grid[x + 1][y] = 1;
            }
        }
    }

    pub fn get(&self, x: usize, y: usize) -> i32 {
        self.grid[x][y]
    }
}
