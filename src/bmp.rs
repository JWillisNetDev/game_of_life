use crate::GameBoard;
use bmp::{Image, Pixel, px};

pub fn into_bmp(game_board: &GameBoard, path: &str) {
    let mut image = Image::new(game_board.width as u32, game_board.height as u32);

    for coord in game_board {
        image.set_pixel(coord.x as u32, coord.y as u32, if coord.cell.is_alive() { px!(255,255,255) } else { px!(0,0,0) });
    }

    image.save(path).unwrap();
}

