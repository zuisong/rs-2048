// 定义常量SIZE为4
pub const SIZE: usize = 4;

// 定义结构体Game2048，包含一个4x4的二维数组board、分数score和随机数生成器rng
pub struct Game2048 {
    board: [[usize; SIZE]; SIZE],
    score: usize,
}

// Game2048的默认实现，初始化board全为0，随机生成两个初始数字，并返回游戏对象
impl Default for Game2048 {
    fn default() -> Self {
        let board = [[0; SIZE]; SIZE];
        let mut game = Self { board, score: 0 };
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
        let (i, j) = empty_tiles[fastrand::usize(0..empty_tiles.len())];
        board[i][j] = if fastrand::f32() <= 0.9 { 2 } else { 4 };
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

    fn move1(&mut self, direction: &Direction) -> bool {
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

        moved
    }

    // 进行消除操作, 并返回是否有数字块移动
    pub fn make_move(&mut self, direction: &Direction) -> bool {
        let moved = self.move1(direction);
        if moved {
            self.add_random_tile();
        }

        moved
    }
}

// 移动方向的枚举类型
#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use crate::{Direction, Game2048};

    #[test]
    fn test_game() {
        let mut game = Game2048::default();
        assert_eq!(game.get_score(), 0);
        assert!(!game.is_game_over());
        game.make_move(&crate::Direction::Left);
    }

    #[test]
    fn test_game2() {
        let board = [[2; 4]; 4];
        let mut game = Game2048 { board, score: 0 };
        assert!(!game.is_game_over());
        assert_eq!(game.get_board(), &[[2; 4]; 4]);

        game.make_move(&crate::Direction::Up);
        game.make_move(&crate::Direction::Left);
        game.make_move(&crate::Direction::Down);
        game.make_move(&crate::Direction::Right);
    }

    #[test]
    fn test_game_over() {
        let board = [
            //
            [2, 4, 2, 4],
            [4, 2, 4, 2],
            [2, 4, 2, 4],
            [4, 2, 4, 2],
        ];
        let mut game = Game2048 { board, score: 0 };

        assert!(game.is_game_over());

        game.add_random_tile();

        assert_eq!(format!("{:?}", Direction::Up), "Up");
        assert_eq!(format!("{:?}", Direction::Down), "Down");
        assert_eq!(format!("{:?}", Direction::Left), "Left");
        assert_eq!(format!("{:?}", Direction::Right), "Right");
    }

    #[test]
    fn test_down() {
        let board = [
            //
            [2, 0, 0, 0],
            [2, 0, 0, 0],
            [4, 0, 0, 0],
            [8, 0, 0, 0],
        ];
        let mut game = Game2048 { board, score: 0 };

        assert!(!game.is_game_over());

        game.move1(&crate::Direction::Down);
        assert_eq!(
            game.get_board(),
            &[
                //
                [0, 0, 0, 0],
                [4, 0, 0, 0],
                [4, 0, 0, 0],
                [8, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn test_right() {
        let board = [
            //
            [2, 0, 0, 0],
            [2, 0, 0, 0],
            [4, 0, 0, 0],
            [2, 2, 4, 8],
        ];
        let mut game = Game2048 { board, score: 0 };

        assert!(!game.is_game_over());

        game.move1(&crate::Direction::Right);
        assert_eq!(
            game.get_board(),
            &[
                //
                [0, 0, 0, 2],
                [0, 0, 0, 2],
                [0, 0, 0, 4],
                [0, 4, 4, 8],
            ]
        );
    }
    #[test]
    fn test_left() {
        let board = [
            //
            [2, 0, 0, 0],
            [2, 0, 0, 0],
            [4, 0, 0, 0],
            [2, 2, 4, 8],
        ];
        let mut game = Game2048 { board, score: 0 };

        assert!(!game.is_game_over());

        game.move1(&crate::Direction::Left);
        assert_eq!(
            game.get_board(),
            &[
                //
                [2, 0, 0, 0],
                [2, 0, 0, 0],
                [4, 0, 0, 0],
                [4, 4, 8, 0],
            ]
        );
    }

    #[test]
    fn test_up() {
        let board = [
            //
            [2, 0, 0, 0],
            [2, 0, 0, 0],
            [4, 0, 0, 0],
            [2, 2, 4, 8],
        ];
        let mut game = Game2048 { board, score: 0 };

        assert!(!game.is_game_over());

        game.move1(&crate::Direction::Up);
        assert_eq!(
            game.get_board(),
            &[
                //
                [4, 2, 4, 8],
                [4, 0, 0, 0],
                [2, 0, 0, 0],
                [0, 0, 0, 0],
            ]
        );
    }
}
