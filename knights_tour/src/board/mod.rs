use std::clone::Clone;
use std::fmt;

// The board dimensions.
pub const NUM_ROWS: usize = 8;
pub const NUM_COLS: usize = NUM_ROWS;
pub const INUM_ROWS: i32 = NUM_ROWS as i32;
pub const INUM_COLS: i32 = NUM_COLS as i32;
pub const UNVISITED: i32 = -1;

pub struct Cell {
    // pub urow: usize,
    // pub ucol: usize,
    pub row: i32,
    pub col: i32,
}

impl Cell {
    pub fn new(r: i32, c: i32) -> Self {
        let val = Self { row: r, col: c };
        return val;
    }

    pub fn add_offset(&mut self, offset: &[i32; 2]) {
        print!("Cell::add_offset: inp Cell = {}, offset = {:?}", self, offset);
        self.row += offset[0];
        self.col += offset[1];
        println!(", out Cell = {}", self);
    }

}

impl std::clone::Clone for Cell {
    fn clone(&self) -> Self {
        Self {
            row: self.row,
            col: self.col,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

pub fn dump_board(board: &[[i32; NUM_COLS]; NUM_ROWS]) {
    println!();
    print!("     ");
    for col in 0..INUM_COLS {
        print!("{:02} ", col);
    }
    println!();
    print!("     ");
    for _col in 0..INUM_COLS {
        print!("== ");
    }
    println!();

    for row in 0..NUM_COLS {
        print!("{:02} | ", row);
        for col in 0..NUM_COLS {
            if board[row as usize][col as usize] == UNVISITED {
                print!(".. ")
            } else {
                print!("{:02} ", board[row][col]);
            }
        }
        println!();
    }
}

pub fn within_limits(row: i32, col: i32) -> bool {
    row >= 0 && row < INUM_ROWS && col >= 0 && col < INUM_COLS
}

pub fn is_cell_within_board(board: &[[i32; NUM_COLS]; NUM_ROWS], cell: &Cell) -> bool {
    cell.row >= 0 && cell.row < INUM_ROWS && cell.col >= 0 && cell.col < INUM_COLS
}

pub fn pick_initial_cell(num_rows: usize, num_cols: usize) -> Cell {
    let mut cell = Cell::new(0, 0);
    cell
}

pub fn store_value_in_cell(board: &mut [[i32; NUM_COLS]; NUM_ROWS], cell: &Cell, value: i32) {
    if is_cell_within_board(board, cell) {
        board[cell.row as usize][cell.col as usize] = value;
    }
}
