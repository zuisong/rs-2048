use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use rand::Rng;
use std::io;
use std::io::{stdout, Write};

const SIZE: usize = 4;

fn main() {
    io::stdout().flush().unwrap();

    let mut board = [[0; SIZE]; SIZE];
    let mut rng = rand::thread_rng();
    add_random_tile(&mut board, &mut rng);
    add_random_tile(&mut board, &mut rng);
    loop {
        println!();
        print_board(&board);
        if is_game_over(&board) {
            println!("Game over!");
        }
        let direction = match get_direction() {
            Some(d) => d,
            None => {
                return;
            }
        };
        if !make_move(&mut board, &direction) {
            println!("Invalid move!");
        } else {
            add_random_tile(&mut board, &mut rng);
        }
    }
}

fn print_board(board: &[[u32; SIZE]; SIZE]) {
    for row in board.iter() {
        for tile in row.iter() {
            print!("{:<5}", tile);
        }
        println!();
    }
}

fn is_game_over(board: &[[u32; SIZE]; SIZE]) -> bool {
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

fn get_direction() -> Option<Direction> {
    let mut stdout = stdout();
    // 启用原始模式
    enable_raw_mode().unwrap();
    loop {
        if let Ok(Event::Key(KeyEvent { code, .. })) = read() {
            let d = match code {
                KeyCode::Up => Direction::Up,
                KeyCode::Down => Direction::Down,
                KeyCode::Left => Direction::Left,
                KeyCode::Right => Direction::Right,
                _ => {
                    break None;
                }
            };
            disable_raw_mode().unwrap();
            println!("{:?}", d);
            return Some(d);
        }

        // 在循环中手动刷新输出，以便在处理事件后立即更新终端界面
        stdout.flush().unwrap();
    }
}

fn make_move(board: &mut [[u32; SIZE]; SIZE], direction: &Direction) -> bool {
    let mut moved = shift_tiles(board, direction);
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

    shift_tiles(board, direction);
    moved
}

fn shift_tiles(board: &mut [[u32; SIZE]; SIZE], direction: &Direction) -> bool {
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

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn add_random_tile(board: &mut [[u32; SIZE]; SIZE], rng: &mut impl Rng) {
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
