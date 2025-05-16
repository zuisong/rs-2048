use console::{Key, Term};
use rs_2048::{Direction, Game2048};

fn main() {
    let mut game = Game2048::default();
    let t = Term::stdout();
    loop {
        let _ = t.clear_screen();
        println!("Score: {}", game.get_score());
        println!();
        print_board(&game);
        if game.is_game_over() {
            println!("Game over! Score: {}", game.get_score());
            return;
        }
        let direction = match get_direction(&t) {
            Some(d) => d,
            None => return,
        };
        if !game.make_move(&direction) {
            println!("Invalid move!");
        }
    }
}

fn get_direction(t: &Term) -> Option<Direction> {
    loop {
        if let Ok(k) = t.read_key() {
            let d = match k {
                Key::ArrowUp => Direction::Up,
                Key::ArrowDown => Direction::Down,
                Key::ArrowLeft => Direction::Left,
                Key::ArrowRight => Direction::Right,
                _ => continue,
            };
            println!("{:?}", d);
            return Some(d);
        }
    }
}

pub fn print_board(game: &Game2048) {
    let cell_width = 5;

    let board = &game.get_board();

    for y in 0..rs_2048::SIZE {
        for x in 0..rs_2048::SIZE {
            let value = board[y][x];
            let out = if value > 0 {
                value.to_string()
            } else {
                Default::default()
            };
            print!("{:^width$}", out, width = cell_width,);
            if x < (rs_2048::SIZE - 1) {
                print!("│");
            }
        }

        println!();

        if y < (rs_2048::SIZE - 1) {
            println!(
                "{}",
                format!("{:─^width$}+", "", width = cell_width).repeat(rs_2048::SIZE),
            );
        }
    }
}
