use super::board::{
    is_cell_within_board, store_value_in_cell, Cell, INUM_COLS, INUM_ROWS, NUM_COLS, NUM_ROWS,
    UNVISITED,
};

pub fn danger_danger() {
    println!("Danger, danger, Will Robinson");
    println!("INUM_ROWS = {}", INUM_ROWS);
    println!("INUM_COLS = {}", INUM_COLS);
    println!("NUM_ROWS = {}", NUM_ROWS);
    println!("NUM_COLS = {}", NUM_COLS);
    println!("UNVISITED = {}", UNVISITED);
}

pub fn find_tour(
    board: &mut [[i32; NUM_COLS]; NUM_ROWS],
    offsets: &[[i32; 2]; 8],
    cur_cell: &Cell,
    num_visited: i32,
) -> bool {
    println!(
        "find_tour: cur_cell = {}, num_visited = {}",
        cur_cell, num_visited
    );

    // check all the cells around the current one
    for offset_idx in 0..offsets.len() {
        let offset = offsets[offset_idx];

        // cur_cell = where we are right now
        // nxt_cell = cur_cell + offset


        let new_cell = Cell::new(cur_cell.row + offset[0],cur_cell.col+offset[1]);
        let on_board: bool = is_cell_within_board(board, cur_cell);
        println!("find_tour: cur_cell = {}, offset_idx = {}, offset = {:?}, new_cell = {}, on board = {}",
                 cur_cell, offset_idx, offsets[offset_idx], &new_cell, on_board);
    }

    return true;
}
