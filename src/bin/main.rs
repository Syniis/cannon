use cannon::board::*;
use cannon::cannon_move::BitMove;
use cannon::cannon_move::MoveWithScore;
use cannon::color::Color::Black;
use cannon::color::Color::White;
use cannon::defs::*;
use cannon::square::Square;
use cannon::tables::init;
use macroquad::prelude::*;
use std::time::Instant;
use vek::Vec2;

const SQUARES: u8 = 8;

#[macroquad::main("Cannon")]
async fn main() {
    init();
    let mut board = Board::start_position();
    let mut last_clicked: Option<Vec2<i32>> = None;
    let mut show_moves = false;
    let mut won: Option<cannon::color::Color> = None;
    let wcsq = board.castle_with_color(White).to_square();
    let wc = Vec2::new(wcsq.file_index() as i32, wcsq.rank_index() as i32);
    let bcsq = board.castle_with_color(Black).to_square();
    let bc = Vec2::new(bcsq.file_index() as i32, bcsq.rank_index() as i32);

    let piece_color_map = |c: cannon::color::Color| match c {
        White => RED,
        Black => YELLOW,
    };
    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::T) {
            show_moves = show_moves ^ true;
        }

        if is_key_pressed(KeyCode::G) {
            let time = Instant::now();
            let MoveWithScore { bit_move: m, score } = board.best_move(8);
            println!("{}, {}, {}", m.src(), m.dst(), board.side_to_move());
            println!("{}", score);
            println!("{}", time.elapsed().as_secs_f32());
            if m == BitMove::null() {
                won = Some(!board.side_to_move());
            } else if m.dst() == board.enemy_castle().to_square() {
                won = Some(board.side_to_move());
            } else {
                board.apply_move(m);
            }
            last_clicked = None;
        }

        if is_key_pressed(KeyCode::U) {
            board.undo_move();
            last_clicked = None;
        }

        let game_size = screen_width().min(screen_height());
        let offset_x = (screen_width() - game_size) / 2. + 10.;
        let offset_y = (screen_height() - game_size) / 2. + 10.;
        let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

        if let Some(color) = won {
            let s = match color {
                White => "White won",
                Black => "Black won",
            };
            println!("{}", s);
            won = None;
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

        for wsq in board.pieces_with_color(White) {
            draw_circle(
                offset_x + sq_size * (wsq.file_index() as f32 + 0.5) as f32,
                offset_y + sq_size * ((SQUARES - wsq.rank_index() - 1) as f32 + 0.5) as f32,
                sq_size / 2.1,
                piece_color_map(White),
            );
        }
        for bsq in board.pieces_with_color(Black) {
            draw_circle(
                offset_x + sq_size * (bsq.file_index() as f32 + 0.5) as f32,
                offset_y + sq_size * ((SQUARES - bsq.rank_index() - 1) as f32 + 0.5) as f32,
                sq_size / 2.5,
                piece_color_map(Black),
            );
        }
        /*
        for x in 0..SQUARES {
            for y in 0..SQUARES {
                let pos = Vec2::new(x as i32, y as i32);
                let sq = Square::make_square(Rank::from_index(y), File::from_index(x));
                let c = board.color_on(sq);

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
        */
        draw_rectangle(
            offset_x + sq_size * (bc.x as f32 + 0.5 * 0.25) as f32,
            offset_y + sq_size * ((SQUARES as i32 - bc.y - 1) as f32 + 0.55 * 0.25) as f32,
            sq_size / 1.25,
            sq_size / 1.25,
            piece_color_map(Black),
        );
        draw_rectangle(
            offset_x - 0.5 + sq_size * (wc.x as f32 + 0.5 * 0.25) as f32,
            offset_y - 0.5 + sq_size * ((SQUARES as i32 - wc.y - 1) as f32 + 0.5 * 0.25) as f32,
            sq_size / 1.25,
            sq_size / 1.25,
            piece_color_map(White),
        );

        if show_moves {
            if let Some(pos) = last_clicked {
                let sq = Square::make_square(
                    Rank::from_index(pos.y as u8),
                    File::from_index(pos.x as u8),
                );
                let moves = board.generate_moves_for(sq);
                for m in moves {
                    draw_rectangle_lines(
                        offset_x + sq_size * m.dst().file_index() as f32,
                        offset_y
                            + sq_size * (SQUARES as i32 - m.dst().rank_index() as i32 - 1) as f32,
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
                    let pos_sq = Square::make_square(
                        Rank::from_index(pos.y as u8),
                        File::from_index(pos.x as u8),
                    );
                    let clicked_sq = Square::make_square(
                        Rank::from_index(clicked_pos.y as u8),
                        File::from_index(clicked_pos.x as u8),
                    );
                    if let Some(bitmove) = board
                        .generate_moves_for(pos_sq)
                        .iter()
                        .find(|m| m.dst() == clicked_sq)
                    {
                        println!("{} -> {}", bitmove.src(), bitmove.dst());
                        if bitmove.dst() == board.enemy_castle().to_square() {
                            won = Some(board.side_to_move());
                        }
                        board.apply_move(*bitmove);
                    }
                    last_clicked = None;
                }
                None => {
                    let clicked_sq = Square::make_square(
                        Rank::from_index(clicked_pos.y as u8),
                        File::from_index(clicked_pos.x as u8),
                    );
                    if let Some(color) = board.color_on(clicked_sq) {
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
