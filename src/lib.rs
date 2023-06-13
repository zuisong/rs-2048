use rand::{rngs::ThreadRng, Rng};

const SIZE: usize = 4;

pub struct Game2048 {
    pub board: [[usize; SIZE]; SIZE],
    rng: ThreadRng,
}

impl Default for Game2048 {
    fn default() -> Self {
        let board = [[0; 4]; 4];
        let mut game = Self {
            board,
            rng: rand::thread_rng(),
        };
        game.add_random_tile();
        game.add_random_tile();
        game
    }
}

impl Game2048 {
    fn add_random_tile(&mut self) {
        let rng = &mut self.rng;

        let board = &mut self.board;
        let mut empty_tiles = Vec::new();
        for i in 0..SIZE {
            for j in 0..SIZE {
                if board[i][j] == 0 {
                    empty_tiles.push((i, j));
                }
            }
        }
        if empty_tiles.is_empty() {
            return;
        }
        let (i, j) = empty_tiles[rng.gen_range(0..empty_tiles.len())];
        board[i][j] = if rng.gen_bool(0.9) { 2 } else { 4 };
    }

    pub fn is_game_over(&self) -> bool {
        let board = &self.board;
        for row in board.iter() {
            for tile in row.iter() {
                if *tile == 0 {
                    return false;
                }
            }
        }
        for i in 0..SIZE {
            for j in 0..SIZE - 1 {
                if board[i][j] == board[i][j + 1] || board[j][i] == board[j + 1][i] {
                    return false;
                }
            }
        }
        true
    }

    fn shift_tiles(&mut self, direction: &Direction) -> bool {
        let board = &mut self.board;
        let mut moved = false;
        match direction {
            Direction::Up => {
                for _ in 0..SIZE {
                    for j in 0..SIZE {
                        for i in 1..(SIZE) {
                            if board[i - 1][j] == 0 {
                                board[i - 1][j] = board[i][j];
                                board[i][j] = 0;
                                moved = true;
                            }
                        }
                    }
                }
            }
            Direction::Down => {
                for _ in 0..SIZE {
                    for j in 0..SIZE {
                        for i in 1..SIZE {
                            if board[i][j] == 0 {
                                board[i][j] = board[i - 1][j];
                                board[i - 1][j] = 0;
                                moved = true;
                            }
                        }
                    }
                }
            }
            Direction::Left => {
                for _ in 0..SIZE {
                    for j in 1..SIZE {
                        for i in 0..SIZE {
                            if board[i][j - 1] == 0 {
                                board[i][j - 1] = board[i][j];
                                board[i][j] = 0;
                                moved = true;
                            }
                        }
                    }
                }
            }
            Direction::Right => {
                for _ in 0..SIZE {
                    for j in 1..SIZE {
                        for i in 0..SIZE {
                            if board[i][j] == 0 {
                                board[i][j] = board[i][j - 1];
                                board[i][j - 1] = 0;
                                moved = true;
                            }
                        }
                    }
                }
            }
        }
        return moved;
    }

    pub fn make_move(&mut self, direction: &Direction) -> bool {
        let mut moved = self.shift_tiles(direction);
        let board = &mut self.board;
        match direction {
            Direction::Up => {
                for j in 0..SIZE {
                    for i in 1..(SIZE) {
                        if board[i][j] == board[i - 1][j] {
                            board[i - 1][j] *= 2;
                            board[i][j] = 0;
                            moved = true;
                        }
                    }
                }
            }
            Direction::Down => {
                for j in (0..SIZE).rev() {
                    for i in (1..SIZE).rev() {
                        if board[i][j] == board[i - 1][j] {
                            board[i][j] *= 2;
                            board[i - 1][j] = 0;
                            moved = true;
                        }
                    }
                }
            }
            Direction::Left => {
                for j in 1..SIZE {
                    for i in 0..(SIZE) {
                        if board[i][j] == board[i][j - 1] {
                            board[i][j - 1] *= 2;
                            board[i][j] = 0;
                            moved = true;
                        }
                    }
                }
            }
            Direction::Right => {
                for j in (1..SIZE).rev() {
                    for i in (0..SIZE).rev() {
                        if board[i][j] == board[i][j - 1] {
                            board[i][j] *= 2;
                            board[i][j - 1] = 0;
                            moved = true;
                        }
                    }
                }
            }
        }

        self.shift_tiles(direction);

        if moved {
            self.add_random_tile();
        }

        moved
    }
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
