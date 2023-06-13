use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use rs_2048::{Direction, Game2048};
use std::io;
use std::io::{stdout, Write};

fn main() {
    io::stdout().flush().unwrap();
    let mut game = Game2048::default();
    loop {
        println!();
        print_board(&game);
        if game.is_game_over() {
            println!("Game over!");
        }
        let direction = match get_direction() {
            Some(d) => d,
            None => {
                return;
            }
        };
        if !game.make_move(&direction) {
            println!("Invalid move!");
        }
    }
}

pub fn print_board(game: &Game2048) {
    let board = &game.board;
    for row in board.iter() {
        for tile in row.iter() {
            print!("{:<5}", tile);
        }
        println!();
    }
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
