#![feature(slice_flatten)]
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{Color, Print, SetBackgroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use rs_2048::{Direction, Game2048};
use std::io::Write;

fn main() {
    std::io::stdout().flush().unwrap();
    let mut game = Game2048::default();
    loop {
        println!();
        print_board(&game);
        println!("{}", code);
        if game.is_game_over() {
            println!("Game over! Score: {}", game.get_score());
        } else {
            println!("Score: {}", game.get_score());
        }
        let direction = match get_direction() {
            Some(d) => d,
            None => return,
        };
        if !game.make_move(&direction) {
            println!("Invalid move!");
        }
    }
}



fn estimate(grid: &[[usize; 4]; 4]) -> i32 {
    let grid: Vec<i32> = grid.flatten().iter().map(|it| *it as i32).collect();

    let mut sum = 0;
    let mut penalty = 0;

    for i in 0..16 {
        sum += grid[i];
        if i % 4 != 3 {
            penalty += (grid[i] - grid[i + 1]).abs();
        }
        if i < 12 {
            penalty += (grid[i] - grid[i + 4]).abs();
        }
    }

    return (sum * 4 - penalty) * 2;
}

fn get_direction(game: &Game2048) -> Option<Direction> {


let mut d = Direction::Left;

for d1 in vec![ Direction::Left, Direction::Right , Direction::Right, Direction::Left]  {
    
}



    let mut stdout = std::io::stdout();
    stdout.flush().unwrap();
}

pub fn print_board(game: &Game2048) {
    let cell_width = 7;

    let board = &game.get_board();

    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();

    for y in 0..4 {
        for x in 0..4 {
            let value = board[y][x];
            let color = cell_color(board[y][x]);
            queue!(
                std::io::stdout(),
                SetBackgroundColor(color),
                Print(format!(
                    "{:^width$}",
                    Some(value)
                        .filter(|x| x > &0)
                        .map_or(Default::default(), |x| x.to_string()),
                    width = cell_width,
                )),
                SetBackgroundColor(Color::Reset),
            )
            .unwrap();

            if x < 4 {
                queue!(std::io::stdout(), Print("│")).unwrap();
            }
        }

        println!();

        if y < 3 {
            queue!(
                std::io::stdout(),
                Print(format!("{:─^width$}+", "", width = cell_width).repeat(4))
            )
            .unwrap();
            println!();
        }
    }
}

fn cell_color(value: usize) -> Color {
    match value {
        2 => Color::from((238, 228, 218)),    // 浅黄色
        4 => Color::from((237, 224, 200)),    // 黄色
        8 => Color::from((242, 177, 121)),    // 浅红色
        16 => Color::from((245, 149, 99)),    // 红色
        32 => Color::from((246, 124, 95)),    // 洋红色
        64 => Color::from((246, 94, 59)),     // 浅洋红色
        128 => Color::from((237, 207, 114)),  // 青色
        256 => Color::from((237, 204, 97)),   // 浅青色
        512 => Color::from((186, 211, 101)),  // 绿色
        1024 => Color::from((173, 191, 119)), // 浅绿色
        2048 => Color::from((109, 179, 218)), // 蓝色
        _ => Color::Black,
    }
}
