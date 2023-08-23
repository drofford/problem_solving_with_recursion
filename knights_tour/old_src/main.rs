use std::time::{Instant};

// The board dimensions.
const NUM_ROWS: usize = 4;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

// Whether we want an open or closed tour.
const REQUIRE_CLOSED_TOUR: bool = false;

// Value to represent a square that we have not visited.
const UNVISITED: i32 = -1;

fn main() {
    // Initialize the vector of move offsets.
    let offsets = [
        [-2, -1],
        [-1, -2],
        [ 2, -1],
        [ 1, -2],
        [-2,  1],
        [-1,  2],
        [ 2,  1],
        [ 1,  2],
    ];

    // Create a NUM_ROWS x NUM_COLS vector with all entries Initialized to UNVISITED.
    let mut board = [[UNVISITED; NUM_COLS]; NUM_ROWS];

    // Start at board[0][0].
    board[0][0] = 0;

    // Try to find a tour.
    let start = Instant::now();
    let success = find_tour(&mut board, &offsets, 0, 0, 1);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);

    if success {
        println!("Success!");
    } else {
        println!("Could not find a tour.");
    }

    dump_board(0, &board);
}

// Try to extend a knight's tour starting at (start_row, start_col).
// Return true or false to indicate whether we have found a solution.
fn find_tour(board: &mut [[i32; NUM_COLS]; NUM_ROWS],
    offsets: & [[i32; 2]; 8],    // 8 possible moves, 2 coordinates each.
    cur_row: i32, cur_col: i32, num_visited: i32) -> bool {

    return xfind_tour(0, board, &offsets, cur_row, cur_col, num_visited);
}

// Try to extend a knight's tour starting at (start_row, start_col).
// Return true or false to indicate whether we have found a solution.
fn xfind_tour(depth: i32, board: &mut [[i32; NUM_COLS]; NUM_ROWS],
    offsets: & [[i32; 2]; 8],    // 8 possible moves, 2 coordinates each.
    cur_row: i32, cur_col: i32, num_visited: i32) -> bool {

    println!("«{}»: ENTERED xfind_tour: num_visited = {}", depth, num_visited);

    if num_visited == INUM_COLS * INUM_ROWS {
        println!("«{}»: We are done A - have visited all squares", depth);
        return true;
    }

    println!("«{}»: looping over all offsets", depth);
    for offset in offsets {
        println!("«{}»: trying offset ({},{})", depth, offset[0], offset[1]);
        // println!("«{}»: BOARD STATE BEFORE TESTING OFFSET ({},{})", depth, offset[0], offset[1]);
        // dump_board(depth, &board);

        let new_row: i32 = cur_row + offset[0];
        let new_col: i32 = cur_col + offset[1];

        print!("«{}»: cur pos = ({},{}), new pos = ({},{}): ", depth, cur_row, cur_col, new_row, new_col);

        if (new_row < 0 || new_row >= INUM_ROWS) || (new_col < 0 || new_col >= INUM_COLS) {
            println!("out of bounds");
            continue;
        }
        if board[new_row as usize][new_col as usize] != UNVISITED {
            println!("already visited");
            continue;
        }
        println!("unvisited");
        println!("«{}»: BOARD STATE BEFORE SELECTING OFFSET ({},{})", depth, offset[0], offset[1]);
        dump_board(depth, &board);
        // let mut msg = String::new();
        // msg.push(new_row.to_string());
        // msg.push(new_col.to_string());
        let val: i32 = num_visited;
        board[new_row as usize][new_col as usize] = val;
        println!("«{}»: unvisited - candidate - setting ({},{}) to {}", depth, new_row, new_col, val);
        println!("«{}»: BOARD STATE AFTER SELECTING OFFSET ({},{}) BUT BEFORE RECURSING",
            depth, offset[0], offset[1]);
        dump_board(depth, &board);
        if xfind_tour(depth + 1, board, &offsets, new_row, new_col, num_visited + 1) {
            println!("«{}»: We are done B - tour found", depth);
            return true;
        }
        board[new_row as usize][new_col as usize] = UNVISITED;
        println!("«{}»: no go - resetting ({},{}) to {}", depth, new_row, new_col, UNVISITED);
    }

    println!("«{}»: All offsets tried: no tour found", depth);
    return false;
}

fn dump_board(depth: i32, board: &[[i32; NUM_COLS]; NUM_ROWS]) {
    print!("«{}»:      ", depth);
    for col in 0..INUM_COLS {
        print!(" {:02}", col);
    }
    println!("");
    print!("«{}»:      ", depth);
    for _col in 0..INUM_COLS {
        print!(" ==");
    }
    println!("");

    for row in 0..NUM_COLS {
        print!("«{}»:  {:02} |", depth, row);
        for col in 0..NUM_COLS {
            // print!("{}{}  ", row, col);
            // let mut ch = '.';
            if board[row as usize][col as usize] == UNVISITED {
                print!(" ..")
            } else {
                print!(" {:02}", board[row as usize][col as usize]);
            }
        }
        println!("");
    }
}
