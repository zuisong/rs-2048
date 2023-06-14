use fltk::{
    app,
    enums::{Color, Event, FrameType, Key},
    frame::*,
    prelude::*,
    window::*,
};

use rs_2048::{Direction, Game2048};

const SCORE_FRAME_WIDTH: i32 = 50;
const _PADDING: i32 = 10;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 370, 410, "2048");

    let mut score_frame = Frame::new(0, 0, wind.width(), SCORE_FRAME_WIDTH, "Score: 0");
    score_frame.set_label_size(30);
    wind.add(&score_frame);

    wind.set_color(Color::White);
    let mut game = Game2048::default();
    draw_board(&mut wind, &game);
    wind.end();
    wind.show();

    let (s, r) = app::channel::<Direction>();

    wind.handle(move |_, ev| {
        match ev {
            Event::KeyDown => match app::event_key() {
                Key::Up => s.send(Direction::Up),
                Key::Down => s.send(Direction::Down),
                Key::Left => s.send(Direction::Left),
                Key::Right => s.send(Direction::Right),
                _ => return false,
            },
            _ => return false,
        };
        true
    });

    while app.wait() {
        if let Some(msg) = r.recv() {
            let moved = game.make_move(&msg);
            if moved {
                wind.redraw();
                draw_board(&mut wind, &game);
            }

            if game.is_game_over() {
                score_frame.set_label(format!("Game Over! Score: {}", game.get_score()).as_str());
            } else {
                score_frame.set_label(format!("Score: {}", game.get_score()).to_string().as_str());
            }
        }
    }
    Ok(())
}

fn draw_board(wind: &mut Window, game: &Game2048) {
    let cell_size = 80;

    let mut x: i32 = _PADDING;
    let mut y = SCORE_FRAME_WIDTH;
    let cell_size_plus_spacing = cell_size + _PADDING;
    for i in 0..4 {
        for j in 0..4 {
            let value = game.board[i][j];
            let value_str = value.to_string();
            let color = cell_color(value);
            let mut frame = Frame::new(x, y, cell_size, cell_size, "");
            frame.set_label_size(30);
            frame.set_frame(FrameType::FlatBox);
            frame.set_color(color);
            frame.set_label(match value {
                0 => "",
                _ => value_str.as_str(),
            });

            wind.add(&frame);
            x += cell_size_plus_spacing;
        }
        x = _PADDING;
        y += cell_size_plus_spacing;
    }
}

fn cell_color(val: usize) -> Color {
    match val {
        0 => Color::White,
        2 => Color::from_rgb(238, 228, 218),
        4 => Color::from_rgb(237, 224, 200),
        8 => Color::from_rgb(242, 177, 121),
        16 => Color::from_rgb(245, 149, 99),
        32 => Color::from_rgb(246, 124, 95),
        64 => Color::from_rgb(246, 94, 59),
        128 => Color::from_rgb(237, 207, 114),
        256 => Color::from_rgb(237, 204, 97),
        512 => Color::from_rgb(237, 200, 80),
        1024 => Color::from_rgb(237, 197, 63),
        2048 => Color::from_rgb(237, 194, 46),
        4096 => Color::from_rgb(237, 190, 28),
        8192 => Color::from_rgb(237, 187, 11),
        16384 => Color::from_rgb(237, 184, 0),
        _ => Color::from_rgb(205, 193, 180),
    }
}
