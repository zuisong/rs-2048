use rand::{rngs::ThreadRng, Rng};

// 定义常量SIZE为4
const SIZE: usize = 4;

// 定义结构体Game2048，包含一个4x4的二维数组board、分数score和随机数生成器rng
pub struct Game2048 {
    board: [[usize; SIZE]; SIZE],
    score: usize,
    rng: ThreadRng,
}

// Game2048的默认实现，初始化board全为0，随机生成两个初始数字，并返回游戏对象
impl Default for Game2048 {
    fn default() -> Self {
        let board = [[0; 4]; 4];
        let mut game = Self {
            board,
            rng: rand::thread_rng(),
            score: 0,
        };
        game.add_random_tile();
        game.add_random_tile();
        game
    }
}

// Game2048的方法实现
impl Game2048 {
    // 获取分数
    pub fn get_score(&self) -> usize {
        self.score
    }

    // 获取游戏面板
    pub fn get_board(&self) -> &[[usize; SIZE]; SIZE] {
        &self.board
    }

    // 随机生成一个数字块
    fn add_random_tile(&mut self) {
        let rng = &mut self.rng;

        let board = &mut self.board;
        let mut empty_tiles = Vec::new();
        for (i, arr) in board.iter().enumerate() {
            for (j, value) in arr.iter().enumerate() {
                if value == &0 {
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

    // 判断游戏是否结束
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

    // 移动数字块, 使数字块紧凑, 但是不消除, 消除数字之间的空格
    fn shift_tiles(&mut self, direction: &Direction) -> bool {
        let board = &mut self.board;
        let mut moved = false;

        let increment = match direction {
            Direction::Up => |i, j| (i - 1, j),
            Direction::Down => |i, j| (i + 1, j),
            Direction::Left => |i, j| (i, j - 1),
            Direction::Right => |i, j| (i, j + 1),
        };
        for _ in 0..SIZE {
            for i in 0..SIZE {
                for j in 0..SIZE {
                    let (next_i, next_j) = increment(i as i32, j as i32);
                    let range = &(0..(SIZE as i32));
                    if !(range.contains(&next_i) && range.contains(&next_j)) {
                        continue;
                    }
                    let next_i = next_i as usize;
                    let next_j = next_j as usize;
                    if board[next_i][next_j] == 0 && board[i][j] != 0 {
                        board[next_i][next_j] = board[i][j];
                        board[i][j] = 0;
                        moved = true;
                    }
                }
            }
        }
        moved
    }

    // 进行消除操作, 并返回是否有数字块移动
    pub fn make_move(&mut self, direction: &Direction) -> bool {
        let mut moved = self.shift_tiles(direction);
        let board = &mut self.board;

        let (x_iter, y_iter): (Vec<_>, Vec<_>) = match direction {
            Direction::Up => ((1..SIZE).collect(), (0..SIZE).collect()),
            Direction::Down => ((1..SIZE).rev().collect(), (0..SIZE).rev().collect()),
            Direction::Left => ((0..SIZE).collect(), (1..SIZE).collect()),
            Direction::Right => ((0..SIZE).rev().collect(), (1..SIZE).rev().collect()),
        };

        let increment = match direction {
            Direction::Up | Direction::Down => |i, j| (i - 1, j),
            Direction::Left | Direction::Right => |i, j| (i, j - 1),
        };

        for &i in x_iter.iter() {
            for &j in y_iter.iter() {
                let (x, y) = increment(i, j);
                if board[i][j] == board[x][y] && board[i][j] != 0 {
                    self.score += board[i][j];
                    board[x][y] *= 2;
                    board[i][j] = 0;
                    moved = true;
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

// 移动方向的枚举类型
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
