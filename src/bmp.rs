use crate::{Cell, GameBoard};
use bmp::{Image, Pixel};

pub fn into_bmp(game_board: &GameBoard, path: &str) {
    let mut image = Image::new(game_board.width as u32, game_board.height as u32);
