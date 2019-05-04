use std::fmt;

pub struct Board {
    pub size: usize,
    pub cells: Vec<Vec<Cell>>,
}

pub enum Cell {
    None,
    Black,
    White,
    Kou,
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
        if !self.is_put(x, y) {
            return false;
        }
        match color {
            Cell::White | Cell::Black => (),
            _ => return false
        }
        self.cells[*x][*y] = color;
        true
    }

    pub fn is_put(&self, x: &usize, y: &usize) -> bool {
        if self.size <= *x || self.size <= *y {
            return false;
        }
        match self.cells[*x][*y] {
            Cell::None => true,
            _ => false
        }
    }

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
