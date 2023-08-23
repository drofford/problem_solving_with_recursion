mod board;
mod tour;

use std::time::Instant;

use board::{
    dump_board, is_cell_within_board, pick_initial_cell, store_value_in_cell, Cell, INUM_COLS,
    INUM_ROWS, NUM_COLS, NUM_ROWS, UNVISITED,
};

use tour::{danger_danger, find_tour};

// Whether we want an open or closed tour.
const REQUIRE_CLOSED_TOUR: bool = false;

fn main() {
    // Initialize the vector of move offsets.
    let offsets = [
        [-2, -1],
        [-1, -2],
        [2, -1],
        [1, -2],
        [-2, 1],
        [-1, 2],
        [2, 1],
        [1, 2],
    ];

    // Create a NUM_ROWS x NUM_COLS vector with all entries Initialized to UNVISITED.
    let mut board = [[board::UNVISITED; board::NUM_COLS]; board::NUM_ROWS];

    println!();
    println!("=============================");
    println!("= I N I T I A L   B O A R D =");
    println!("=============================");
    board::dump_board(&board);

    let mut cur_cell = pick_initial_cell(NUM_ROWS, NUM_COLS);

    store_value_in_cell(&mut board, &cur_cell, 1);

    tour::danger_danger();
    let r = tour::find_tour(&mut board, &offsets, &cur_cell, 1);
    // println("top level call to find_tour returned {}", r);
}
