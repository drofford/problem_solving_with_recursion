// The board dimensions.
pub const NUM_ROWS: usize = 8;
pub const NUM_COLS: usize = NUM_ROWS;
pub const INUM_ROWS: i32 = NUM_ROWS as i32;
pub const INUM_COLS: i32 = NUM_COLS as i32;
pub const UNVISITED: i32 = -1;

pub struct Cell {
    pub row: usize,
    pub col: usize,
}

impl Cell  {
    pub fn new() -> Self {
        return Self { row: 0, col: 0 };
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

pub fn is_cell_within_board(board: &[[i32; NUM_COLS]; NUM_ROWS], cell: &Cell) -> bool {
    cell.row >= 0 && cell.row < NUM_ROWS && cell.col >=0 && cell.col < NUM_COLS
}

pub fn store_value_in_cell(board: &mut [[i32; NUM_COLS]; NUM_ROWS], cell: &Cell, value: i32) {
    if is_cell_within_board(board, cell) {
        board[cell.row][cell.col] = value;
    }
}
