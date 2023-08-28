use std::time::{Instant};

// The board dimensions.
const NUM_ROWS: usize = 8;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

fn main() {
    // Create a NUM_ROWS x NUM_COLS array with all entries Initialized to UNVISITED.
    let mut board = [['.'; NUM_COLS]; NUM_ROWS];

    let start = Instant::now();
    let success = place_queens_1(&mut board, 0, 0);
    // let success = place_queens_2(& mut board, 0, 0, 0);
    //let success = place_queens_3(& mut board);
    let duration = start.elapsed();

    println!("Time: {:?}", duration);

    if success {
        println!("Success!");
    } else {
        println!("Could not find a tour.");
    }

    dump_board(&mut board);
}

// Return true if the board is legal.
fn board_is_legal(board: &mut [[char; NUM_COLS]; NUM_ROWS]) -> bool {

    // every row
    for row in 0..INUM_ROWS {
        if !series_is_legal(board, row, 0, 0, 1) {
            return false;
        }
    }

    // every column
    for col in 0..INUM_COLS {
        if !series_is_legal(board, 0, col, 1, 0) {
            return false;
        }
    }

    for row in 0..INUM_ROWS {
        if !series_is_legal(board, row, 0, 1, 1) {
            return false;
        }
    }

    // every diagnonal
    for col in 0..INUM_COLS {
        if !series_is_legal(board, 0, col, 1, 1) {
            return false;
        }
    }

    return true;
}

fn dump_board(board: &mut [[char; NUM_COLS]; NUM_ROWS]) {
    for row in 0..NUM_COLS {
        for col in 0..NUM_COLS {
            print!("{:1} ", board[row][col]);
        }
        println!();
    }
}

fn place_queens_1(board: &mut [[char; NUM_COLS]; NUM_ROWS], row: i32, col: i32) -> bool {
    return true;
}

// Return true if this series of squares contains at most one queen.
fn series_is_legal(board: &mut [[char; NUM_COLS]; NUM_ROWS],
    r0: i32, c0: i32, dr: i32, dc: i32) -> bool {

    let mut count: i32 = 0;

    let mut r = r0;
    let mut c = c0;

    while r < INUM_ROWS && c < INUM_COLS  {
        if board[r as usize][c as usize] == 'Q' {
            count += 1;
        }
        r += dr;
        c += dc;
    }

    return count < 2;
}

