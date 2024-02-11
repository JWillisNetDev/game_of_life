use game_of_life::{GameBoard, bmp};

fn main() {
    let mut board = GameBoard::new(100, 100);
    let board = board.enable(49, 49)
        .enable(50, 49)
        .enable(51, 49)
        .enable(51, 50)
        .enable(51, 51)
        .enable(50, 51)
        .enable(49, 51)
        .enable(49, 50);

    let root = "output";
    bmp::into_bmp(board, get_path(root, 0).as_str());
    for i in 1..10 {
        board.next();
        bmp::into_bmp(board, get_path(root, i).as_str());
    }
}

fn get_path(root: &str, count: usize) -> String {
    let mut path = String::new();

    path.push_str(root);
    if count > 0 {
        path.push_str(format!("_({count})").as_str());
    }
    path.push_str(".bmp");

    path
}