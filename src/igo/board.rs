use std::fmt;
use std::collections::HashSet;
use std::rc::Rc;
use core::borrow::Borrow;

#[derive(Clone)]
pub struct Board {
    pub size: usize,
    pub cells: Vec<Vec<Cell>>,
    pub poss: Vec<(usize, usize, Cell)>,
}

#[derive(PartialEq, Clone)]
pub enum Cell {
    None,
    Black,
    White,
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
            poss: vec![],
        }
    }

    pub fn put(&mut self, x: &usize, y: &usize, color: Cell) -> bool {
        match color {
            Cell::White | Cell::Black => (),
            _ => return false
        }
        let removeable_pos = self.calc_removeable_pos(x, y, &color);
        let mut is_kou = false;
        removeable_pos.iter().for_each(|&(x, y)| {
            if removeable_pos.len() == 1 {
                let pos = &self.poss[0];
                if pos.0 == x && pos.1 == y {
                    is_kou = true;
                    return;
                }
            }
            self.cells[x][y] = Cell::None;
        });
        if is_kou { return false; }
        if removeable_pos.len() == 0 && !self.is_put(x, y, &color) {
            return false;
        }
        self.cells[*x][*y] = color.clone();
        self.poss.push((*x, *y, color));
        true
    }


    pub fn calc_removeable_pos(&self, put_x: &usize, put_y: &usize, color: &Cell) -> Vec<(usize, usize)> {
        BoardDirectionIter::new(*put_x, *put_y, self).flat_map(|(x, y)| {
            let mut queue = Vec::with_capacity(self.size);
            let mut i = 0;
            let mut rem = vec![vec![false; self.size]; self.size];
            let mut removeable = true;
            if self.cells[x][y] == color.to_enemy_cell() {
                queue.push((x, y));
            } else {
                return Vec::new();
            }
            rem[*put_x][*put_y] = true;
            rem[x][y] = true;

            loop {
                if queue.len() <= i { break; }
                let x = queue[i].0;
                let y = queue[i].1;
                BoardDirectionIter::new(x, y, self).for_each(|(x, y)| {
                    if rem[x][y] { return; }
                    let cell = &self.cells[x][y];
                    if cell == color {
                        // do nothing
                    } else if *cell == color.to_enemy_cell() {
                        queue.push((x, y));
                    } else {
                        removeable = false;
                    }
                    rem[x][y] = true;
                });
                if !removeable { break; }
                i += 1;
            }
            if removeable {
                queue
            } else {
                Vec::new()
            }
        }).collect()
    }

    pub fn is_put(&self, x: &usize, y: &usize, color: &Cell) -> bool {
        if self.size <= *x || self.size <= *y {
            return false;
        }
        match self.cells[*x][*y] {
            Cell::None => (),
            _ => return false
        }
        let remove_poss = self.calc_removeable_pos(x, y, color);
        // check kou
        if remove_poss.len() == 1 {
            let pos = remove_poss[0];
            if pos.0 == *x && pos.1 == *y {
                return false;
            }
        }
        if remove_poss.len() > 0 { return true; }
        let mut queue = Vec::with_capacity(self.size);
        let mut i = 0;
        let mut rem = vec![vec![false; self.size]; self.size];
        let mut ok = false;
        BoardDirectionIter::new(*x, *y, self).for_each(|(x, y)| {
            if self.cells[x][y] == Cell::None {
                ok = true;
            }
            if self.cells[x][y] == *color {
                queue.push((x, y));
            }
            rem[x][y] = true;
        });
        if ok { return true; }
        rem[*x][*y] = true;
        loop {
            if queue.len() <= i { break; }
            let mut ok = false;
            let x = queue[i].0;
            let y = queue[i].1;
            BoardDirectionIter::new(x, y, self).for_each(|(x, y)| {
                if rem[x][y] { return; }
                let cell = &self.cells[x][y];
                if *cell == *color {
                    queue.push((x, y));
                } else if *cell == color.to_enemy_cell() {
//                    do nothing
                } else {
                    ok = true;
                }
                rem[x][y] = true;
            });
            if ok { return true; }
            i += 1;
        }
        false
    }

    pub fn pass(&mut self) {
    }

    #[cfg(test)]
    pub fn set_from_str(&mut self, s: String) {
        s.chars().enumerate().for_each(|(i, c)| {
            let x = i % (self.size + 1);
            let y = i / (self.size + 1);
            if x == self.size { return; }
            self.cells[y][x] = match c {
                'B' => Cell::Black,
                'W' => Cell::White,
                _ => Cell::None
            }
        })
    }
}

pub struct BoardDirectionIter {
    x: usize,
    y: usize,
    board: Board,
    direction_iter: Vec<(usize, usize)>,
    i: usize,
}

impl BoardDirectionIter {
    pub fn new(x: usize, y: usize, board: &Board) -> BoardDirectionIter {
        let mut direction_iter = Vec::with_capacity(4);
        if x > 0 { direction_iter.push((x - 1, y)) }
        if x < board.size - 1 { direction_iter.push((x + 1, y)) }
        if y > 0 { direction_iter.push((x, y - 1)) }
        if y < board.size - 1 { direction_iter.push((x, y + 1)) }
//        TODO: doing clone maybe late
        BoardDirectionIter {
            x,
            y,
            board: (*board).clone(),
            direction_iter,
            i: 0,
        }
    }
}

impl Iterator for BoardDirectionIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.direction_iter.len() > self.i {
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
            Cell::None => " ",
        }.to_string()
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Cell::Black => "Black",
            Cell::White => "White",
            Cell::None => "None",
        };
        write!(f, "{}", s)
    }
}
