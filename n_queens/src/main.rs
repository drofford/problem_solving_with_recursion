use std::time::Instant;

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
    // let success = place_queens_3(& mut board);
    let duration = start.elapsed();

    println!("Time: {:?}", duration);

    if success {
        println!("Success!");
    } else {
        println!("Could not find a tour.");
    }

    dump_board(&mut board);
}

// Return true if the board is legal and a solution.
fn board_is_a_solution(board: &mut [[char; NUM_COLS]; NUM_ROWS]) -> bool {
    if board_is_legal(board) {
        let mut count = 0;
        for row in 0..NUM_ROWS {
            for col in 0..NUM_COLS {
                if board[row][col] == 'Q' {
                    count += 1;
                }
            }
        }
        if count == NUM_ROWS {
            return true;
        }
    }
    false
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

    // for row 0, check each diagonal
    for col in 0..INUM_COLS {
        if !series_is_legal(board, 0, col, 1, 1) {
            return false;
        }
    }
    
    // for col 0, check each diagonal
    for row in 0..INUM_ROWS {
        if !series_is_legal(board, row, 0, 1, 1) {
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

// Try placing a queen at position [r][c].
// Return true if we find a legal board.
fn place_queens_1(board: &mut [[char; NUM_COLS]; NUM_ROWS], r: i32, c: i32) -> bool {

    if r >= INUM_ROWS {
        return board_is_a_solution(board);
    }

    // check board[r][c]
    //  -- place a queen or NOT place a queen?

    let mut next_r = r;
    let mut next_c = c + 1;
    if next_c >= INUM_COLS {
        next_r += 1;
        next_c = 0;
    }

    // (1) do not place a Queen at [r][c] and check
    if place_queens_1(board, next_r, next_c) {
        return true; // solution found
    }

    // (2) do place a Queen at [r][c] and check
    board[r as usize][c as usize] = 'Q';
    if place_queens_1(board, next_r, next_c) {
        return true; // solution found
    }

    // uh oh -no solution
    board[r as usize][c as usize] = '.';
    return false;
}

// Return true if this series of squares contains at most one queen.
fn series_is_legal(
    board: &mut [[char; NUM_COLS]; NUM_ROWS],
    r0: i32,
    c0: i32,
    dr: i32,
    dc: i32,
) -> bool {
    let mut r = r0;
    let mut c = c0;
    let mut found: bool = false;

    while r < INUM_ROWS && c < INUM_COLS {
        if board[r as usize][c as usize] == 'Q' {
            if found {
                return false;
            } else {
                found = true;
            }
        }
        r += dr;
        c += dc;
    }
    true
}
