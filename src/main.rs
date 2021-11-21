mod board;
mod cannon_move;
mod grid;

use board::*;
use cannon_move::CannonMove;
use macroquad::prelude::*;
use vek::Vec2;

const SQUARES: u8 = 10;

#[macroquad::main("Cannon")]
async fn main() {
    let castles = [Vec2::new(0, 0), Vec2::new(9, 9)];
    let mut board = Board::new(&castles);
    let mut last_clicked: Option<Vec2<i32>> = None;
    let mut show_moves = false;
    let mut won: Option<board::Color> = None;

    let piece_color_map = |c: board::Color| match c {
        board::Color::White => RED,
        board::Color::Black => YELLOW,
    };
    loop {
        clear_background(BLACK);

        if board.castle_dead(board::Color::Black) {
            assert!(!board.is_empty(board.white_castle()));
            won = Some(board::Color::White);
        }
        if board.castle_dead(board::Color::White) {
            assert!(!board.is_empty(board.black_castle()));
            won = Some(board::Color::Black);
        }

        if is_key_pressed(KeyCode::T) {
            show_moves = show_moves ^ true;
        }

        let game_size = screen_width().min(screen_height());
        let offset_x = (screen_width() - game_size) / 2. + 10.;
        let offset_y = (screen_height() - game_size) / 2. + 10.;
        let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

        if let Some(color) = won {
            let s = match color {
                board::Color::White => "White won",
                board::Color::Black => "Black won",
            };
            draw_text(s, game_size / 2.0, game_size / 2.0, 35.0, WHITE);
            next_frame().await;
            continue;
        }

        draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

        for i in 1..SQUARES {
            draw_line(
                offset_x,
                offset_y + sq_size * i as f32,
                screen_width() - offset_x,
                offset_y + sq_size * i as f32,
                2.,
                LIGHTGRAY,
            );
        }

        for i in 1..SQUARES {
            draw_line(
                offset_x + sq_size * i as f32,
                offset_y,
                offset_x + sq_size * i as f32,
                screen_height() - offset_y,
                2.,
                LIGHTGRAY,
            );
        }

        for x in 0..SQUARES {
            for y in 0..SQUARES {
                let pos = Vec2::new(x as i32, y as i32);
                let c = board.color_at(pos);

                if let Some(color) = c {
                    draw_circle(
                        offset_x + sq_size * (x as f32 + 0.5) as f32,
                        offset_y + sq_size * ((SQUARES - y - 1) as f32 + 0.5) as f32,
                        sq_size / 2.1,
                        piece_color_map(color),
                    );
                }
            }
        }
        draw_rectangle(
            offset_x + sq_size * (board.black_castle().x as f32 + 0.5 * 0.25) as f32,
            offset_y
                + sq_size
                    * ((SQUARES as i32 - board.black_castle().y - 1) as f32 + 0.55 * 0.25) as f32,
            sq_size / 1.25,
            sq_size / 1.25,
            piece_color_map(board::Color::Black),
        );
        draw_rectangle(
            offset_x - 0.5 + sq_size * (board.white_castle().x as f32 + 0.5 * 0.25) as f32,
            offset_y - 0.5
                + sq_size
                    * ((SQUARES as i32 - board.white_castle().y - 1) as f32 + 0.5 * 0.25) as f32,
            sq_size / 1.25,
            sq_size / 1.25,
            piece_color_map(board::Color::White),
        );

        if show_moves {
            if let Some(pos) = last_clicked {
                let moves = board.enumerate_moves_for(pos);

                for CannonMove {
                    source: _,
                    dest,
                    is_shot: _,
                } in moves.iter()
                {
                    draw_rectangle_lines(
                        offset_x + sq_size * dest.x as f32,
                        offset_y + sq_size * (SQUARES as i32 - dest.y - 1) as f32,
                        sq_size,
                        sq_size,
                        5.0,
                        GREEN,
                    );
                }
            }
        }
        if let Some(pos) = last_clicked {
            draw_rectangle_lines(
                offset_x + sq_size * pos.x as f32,
                offset_y + sq_size * (SQUARES as i32 - pos.y - 1) as f32,
                sq_size,
                sq_size,
                5.0,
                RED,
            );
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            let sqx = ((x - offset_x) / sq_size).floor() as i32;
            let sqy = ((y - offset_y) / sq_size).floor() as i32;
            let clicked_pos = Vec2::new(sqx, SQUARES as i32 - sqy - 1);
            match last_clicked {
                Some(pos) => {
                    if let Some(m) = board.valid_move(pos, clicked_pos) {
                        board.make_move(m);
                    }
                    last_clicked = None;
                }
                None => {
                    if let Some(color) = board.color_at(clicked_pos) {
                        if color == board.side_to_move() {
                            last_clicked = Some(clicked_pos);
                        }
                    }
                }
            }
        }

        next_frame().await
    }
}
