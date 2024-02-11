mod bmp;
mod iter;

use std::fmt::Formatter;

#[derive(Clone, Copy, Debug, PartialEq,)]
pub struct Cell(bool);

impl Cell {
    pub fn is_alive(&self) -> bool {
        self.0
    }
}

#[derive(Clone)]
pub struct GameBoard {
    width: usize,
    height: usize,
    state: Vec::<Cell>
}

impl std::fmt::Debug for GameBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Width: {}, Height: {}", self.width, self.height)?;
        for y in 0..self.height {
            let slice = &self.state[(y * self.width)..(y * self.width + self.width)];
            let text = slice.iter()
                .map(|cell| match cell {
                    Cell(true) => "1",
                    _ => "0",
                })
                .collect::<Vec::<&str>>()
                .join(", ");
            writeln!(f, "{text}")?;
        }
        Ok(())
    }
}

impl GameBoard {
    pub fn new(width: usize, height: usize) -> Self {
        let capacity = width * height;
        let board = vec![Cell(false); capacity];

        Self { width, height, state: board }
    }

    pub fn size(&self) -> usize {
        self.width * self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Cell> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.state[y * self.width + x])
        }
    }

    pub fn enable(&mut self, x: usize, y: usize) -> &mut Self {
        self.set(x, y, Cell(true));
        self
    }

    pub fn disable(&mut self, x: usize, y: usize) -> &mut Self {
        self.set(x, y, Cell(false));
        self
    }

    fn set(&mut self, x: usize, y: usize, value: Cell) {
        if let Some(cell) = self.state.get_mut(y * self.width + x) {
            *cell = value;
        }
    }

    pub fn next(&mut self) {
        /*  Rules of Game of Life
            1. Any live cell with fewer than 2 neighbors dies.
            2. Any live cell with 2 or 3 neighbors lives.
            3. Any live cell with more than 3 neighbors dies.
            4. Any dead cell with exactly 3 neighbors becomes a live cell. */

        let size = self.size();
        let mut next_state = vec![Cell(false); size];
        for i in 0..size {
            let x = i % self.width;
            let y = i / self.width;
            let alive = self.get_alive_neighbors(x, y);
            let cell = self.state[i];

            if cell.is_alive() && alive < 2 {
               next_state[i] = Cell(false);
            } else if cell.is_alive() && alive > 3 {
                next_state[i] = Cell(false);
            } else if !cell.is_alive() && alive == 3 {
                next_state[i] = Cell(true);
            } else {
                next_state[i] = cell;
            }
        }

        self.state = next_state;
    }
    fn get_neighbors(&self, x: usize, y: usize) -> Option<[Option<Cell>; 9]> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let mut neighbors = [None; 9];
        for j in 0..3 {
            for k in 0..3 {
                if (x == 0 && j == 0) || (y == 0 && k == 0) {
                    neighbors[j * 3 + k] = Some(Cell(false));
                }
                else if j == 1 && k == 1 {
                    neighbors[j * 3 + k] = None;
                }
                else {
                    neighbors[j * 3 + k] = self.get(x + j - 1, y + k - 1);
                }
            }
        }

        Some(neighbors)
    }

    fn get_alive_neighbors(&self, x: usize, y: usize) -> usize {
        if let Some(neighbors) = self.get_neighbors(x, y) {
            neighbors.iter()
                .filter(|cell| **cell == Some(Cell(true)))
                .count()
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_fills_with_0() {
        let width = 4;
        let height = 4;
        let size = width * height;

        let board = GameBoard::new(width, height);

        assert_eq!(size, board.state.capacity());
        assert_eq!(size, board.size());
        for cell in board.state {
            assert_eq!(Cell(false), cell);
        }
    }

    #[test]
    fn it_gets_neighbors() {
        let board = from_2d_array(&[
            &[Cell(true), Cell(false), Cell(true)],
            &[Cell(false), Cell(true), Cell(false)],
            &[Cell(true), Cell(false), Cell(true)],
        ]);

        let neighbors = board.get_neighbors(1, 1).unwrap();
        assert_eq!(neighbors[0], Some(Cell(true)));
        assert_eq!(neighbors[1], Some(Cell(false)));
        assert_eq!(neighbors[2], Some(Cell(true)));
        assert_eq!(neighbors[3], Some(Cell(false)));
        assert_eq!(neighbors[4], None);
        assert_eq!(neighbors[5], Some(Cell(false)));
        assert_eq!(neighbors[6], Some(Cell(true)));
        assert_eq!(neighbors[7], Some(Cell(false)));
        assert_eq!(neighbors[8], Some(Cell(true)));
    }

    #[test]
    fn it_enables() {
        let mut board = GameBoard::new(3, 3);
        board.enable(1, 1);

        assert_eq!(Some(Cell(true)), board.get(1, 1));
    }

    #[test]
    fn it_disables() {
        let mut board = GameBoard::new(3, 3);
        for i in board.state.iter_mut() {
            *i = Cell(true);
        }

        board.disable(1, 1);
        assert_eq!(Some(Cell(false)), board.get(1, 1))
    }

    #[test]
    fn it_gets_next_state() {
        let mut board = GameBoard::new(3, 3);
        board.enable(0, 0)
            .enable(1, 0)
            .enable(0, 1)
            .enable(1, 1)
            .enable(2, 2);

        let expected_state = &[
            &[Cell(true ), Cell(true ), Cell(false)],
            &[Cell(true ), Cell(false), Cell(true )],
            &[Cell(false), Cell(true ), Cell(false)],
        ];

        dbg!(board.clone());
        board.next();

        dbg!(board.clone());
        for i in 0..9 {
            assert_eq!(expected_state[i%3][i/3], board.state[i]);
        }
    }

    fn from_2d_array(arr: &[&[Cell]]) -> GameBoard {
        let height = arr.len();
        let width = arr[0].len();
        let mut gameboard = GameBoard::new(width, height);

        for j in 0..arr.len() {
            for k in 0..arr[j].len() {
                let cell = gameboard.state.get_mut(j * width + k).unwrap();
                *cell = arr[j][k].to_owned();
            }
        }
        gameboard
    }
}