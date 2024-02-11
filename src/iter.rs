use crate::{Cell, GameBoard};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Coordinate<'a> {
    pub x: usize,
    pub y: usize,
    pub cell: &'a Cell,
}

impl<'a> Iterator for CoordinateIter<'a> {
    type Item = Coordinate<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.size {
            None
        } else {
            let cell = &self.game_board.state[self.index];
            let item = Coordinate {
                y: self.index / self.game_board.width,
                x: self.index % self.game_board.width,
                cell,
            };
            self.index += 1;
            Some(item)
        }
    }
}

pub struct CoordinateIter<'a> {
    size: usize,
    index: usize,
    game_board: &'a GameBoard,
}

impl<'a> IntoIterator for &'a GameBoard {
    type Item = Coordinate<'a>;
    type IntoIter = CoordinateIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CoordinateIter {
            size: self.size(),
            index: 0,
            game_board: self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let width = 3;
        let height = 1;
        let mut board = GameBoard::new(width, height);
        board.enable(1, 0);

        let mut iter = board.into_iter();
        assert_eq!(3, iter.size); // Right size
        assert_eq!(0, iter.index); // Right starting index

        let board = &board as *const _;
        assert_eq!(board, iter.game_board as *const _); // Same reference

        let actual = iter.next().unwrap();
        assert_eq!(
            Coordinate {
                x: 0,
                y: 0,
                cell: &Cell(false),
            },
            actual
        );

        let actual = iter.next().unwrap();
        assert_eq!(
            Coordinate {
                x: 1,
                y: 0,
                cell: &Cell(true),
            },
            actual
        );

        let actual = iter.next().unwrap();
        assert_eq!(
            Coordinate {
                x: 2,
                y: 0,
                cell: &Cell(false),
            },
            actual
        );

        assert!(iter.next().is_none());
    }
}
