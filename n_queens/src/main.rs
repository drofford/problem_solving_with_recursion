use std::time::{Instant};

// The board dimensions.
const NUM_ROWS: usize = 5;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

static mut NUM_MOVES: u32 = 0;

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
            println!("board_is_a_solution [0] count = {}, NUM_ROWS = {}", count, NUM_ROWS);
            println!("board_is_a_solution [1] returns TRUE");
            return true;
        } else {
            println!("board_is_a_solution [2] count = {}, NUM_ROWS = {}", count, NUM_ROWS);
            println!("board_is_a_solution [3] returns false");
        }
    }
    println!("board_is_a_solution [4] returns false");
    false
}

// Return true if the board is legal.
fn board_is_legal(board: &mut [[char; NUM_COLS]; NUM_ROWS]) -> bool {

    // every row
    for row in 0..INUM_ROWS {
        if !series_is_legal(board, row, 0, 0, 1) {
            println!("board_is_legal [1] returns false");
            return false;
        }
    }

    // every column
    for col in 0..INUM_COLS {
        if !series_is_legal(board, 0, col, 1, 0) {
            println!("board_is_legal [2] returns false");
            return false;
        }
    }

    for row in 0..INUM_ROWS {
        if !series_is_legal(board, row, row, 1, 1) {
            println!("board_is_legal [3] returns false when row={}", row);
            return false;
        }
    }

    // every diagnonal
    for col in 0..INUM_COLS {
        if !series_is_legal(board, 0, col, 1, 1) {
            println!("board_is_legal [4] returns false");
            return false;
        }
    }

    println!("board_is_legal [5] returns TRUE");
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

    unsafe {
        NUM_MOVES += 1;
        println!("place_queens_1 [0] This is the state of the board before move #{} with r={}, c={}:", 
            NUM_MOVES, r, c);
    };

    dump_board(board);

    if r >= INUM_ROWS {
        println!("place_queens_1 [1] returns TRUE (r = {})", r);
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
        println!("place_queens_1 [2] returns TRUE");
        return true; // solution found
    }

    // (2) do place a Queen at [r][c] and check
    board[r as usize][c as usize] = 'Q';
    if place_queens_1(board, next_r, next_c) {
        println!("place_queens_1 [3] returns TRUE");
        return true; // solution found
    }

    // uh oh -no solution
    board[r as usize][c as usize] = '.';
    println!("place_queens_1 [4] returns false");
    return false;
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
            println!("series_is_legal [0] found Q at r={}, c={}, count={}", r, c, count);
        }
        r += dr;
        c += dc;
    }

    let result = count < 2;
    println!("series_is_legal [1] r0={}, c0={}, dr={}, dc={}, r={}, c={}, count={}, return {}",
        r0, c0, dr, dc, r, c, count, result);
    return result;
}

