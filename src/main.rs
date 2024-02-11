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

    let mut name_factory = IncrementalNameFactory::new("game_board".to_string(), ".bmp".to_string());

    bmp::into_bmp(board, name_factory.next().as_str());
    for i in 0..=10 {
        board.next();
        bmp::into_bmp(board, name_factory.next().as_str());
    }
}

struct IncrementalNameFactory {
    root: String,
    extension: String,
    count: usize,
}

impl IncrementalNameFactory {
    pub fn new(root: String, extension: String) -> Self {
        IncrementalNameFactory { root, extension, count: 0 }
    }

    pub fn next(&mut self) -> String {
        let mut str = String::new();
        str.push_str(self.root.as_str());
        if self.count > 0 {
            str.push_str(format!("_({})", self.count).as_str());
        }
        str.push_str(self.extension.as_str());
        self.count += 1;
        str
    }
}