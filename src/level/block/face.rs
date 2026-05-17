#[derive(Clone, Copy, Debug)]
pub enum BlockFace {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

impl BlockFace {
    pub fn offset(&self) -> (i32, i32, i32) {
        match self {
            BlockFace::Up => (0, 1, 0),
            BlockFace::Down => (0, -1, 0),
            BlockFace::North => (-1, 0, 0),
            BlockFace::South => (1, 0, 0),
            BlockFace::East => (0, 0, -1),
            BlockFace::West => (0, 0, 1),
        }
    }
}
