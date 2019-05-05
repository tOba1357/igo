use std::fmt;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Clone)]
pub struct Board {
    pub size: usize,
    pub cells: Vec<Vec<Cell>>,
}

#[derive(PartialEq, Clone)]
pub enum Cell {
    None,
    Black,
    White,
    Kou,
}

impl Cell {
    pub fn to_enemy_cell(&self) -> Cell {
        match self {
            Cell::Black => Cell::White,
            Cell::White => Cell::Black,
            _ => panic!("invalid argment")
        }
    }
}

impl Board {
    pub fn new(size: usize) -> Board {
        let mut cells = Vec::with_capacity(size);
        for _ in 0..size {
            let mut row = Vec::with_capacity(size);
            for _ in 0..size {
                row.push(Cell::None)
            }
            cells.push(row)
        }
        Board {
            size,
            cells,
        }
    }

    pub fn put(&mut self, x: &usize, y: &usize, color: Cell) -> bool {
        if !self.is_put(x, y, &color) {
            return false;
        }
        match color {
            Cell::White | Cell::Black => (),
            _ => return false
        }
        self.cells[*x][*y] = color;
        true
    }

    pub fn is_put(&self, x: &usize, y: &usize, color: &Cell) -> bool {
        if self.size <= *x || self.size <= *y {
            return false;
        }
        match self.cells[*x][*y] {
            Cell::None => (),
            _ => return false
        }
        let mut queue = Vec::with_capacity(self.size);
        let mut i = 0;
        let mut rem = vec![false; self.size * self.size];
        queue.push((x, y));
        rem[self.tow_dir_to_one_dir(x, y)] = true;
        loop {
            if queue.len() <= i { break; }
            let mut ok = false;
            let x = queue[i].0;
            let y = queue[i].1;
            BoardDirectionIter::new(*x, *y, self).for_each(|(x, y)| {
                let cell = &self.cells[x][y];
                if *cell == *color {
//                    TODO

                } else if *cell == color.to_enemy_cell() {
//                    TODO

                } else {
                    ok = true;
                }
            });
            if ok { return true }
            i += 1;
        }
        true
    }

    fn tow_dir_to_one_dir(&self, x: &usize, y: &usize) -> usize { x + y * self.size }

    pub fn pass(&mut self) {
        for i in 0..self.size {
            for j in 0..self.size {
                match self.cells[i][j] {
                    Cell::Kou => self.cells[i][j] = Cell::None,
                    _ => ()
                }
            }
        }
    }
}

pub struct BoardDirectionIter {
    x: usize,
    y: usize,
    board: Board,
    direction_iter: Vec<(usize, usize)>,
    i: usize
}

impl BoardDirectionIter {
    pub fn new(x: usize, y :usize, board: &Board) -> BoardDirectionIter {
        let mut direction_iter = Vec::with_capacity(4);
        if x > 0 { direction_iter.push((x - 1, y)) }
        if x < board.size - 1 { direction_iter.push((x + 1, y)) }
        if y > 0 { direction_iter.push((x, y - 1)) }
        if y < board.size - 1 { direction_iter.push((x, y + 1))}
//        TODO: doing clone maybe late
        BoardDirectionIter {
            x,
            y,
            board: (*board).clone(),
            direction_iter,
            i: 0
        }
    }
}

impl Iterator for BoardDirectionIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.direction_iter.len() < self.i {
            let r = Some(self.direction_iter[self.i]);
            self.i += 1;
            r
        } else {
            None
        }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.cells.iter().map(|row|
            row.iter().map(|cell| cell.to_string())
                .collect::<Vec<String>>().join("") + "\n"
        ).collect::<Vec<String>>().join("");
        write!(f, "{}", s)
    }
}

impl Cell {
    pub fn to_string(&self) -> String {
        match self {
            Cell::Black => "B",
            Cell::White => "W",
            Cell::Kou => "K",
            Cell::None => " ",
        }.to_string()
    }
}
impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Cell::Black =>  "Black",
            Cell::White => "White",
            Cell::Kou => "Kou",
            Cell::None => "None",
        };
        write!(f, "{}", s)
    }
}
