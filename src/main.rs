mod piece;
mod board;

use piece::*;
use board::*;

fn main() {
    let mut board = Board::<6, 6>::new();

    // board.is_playable();
    board.cells.search_blocks_matchable();
    board.render();

    // loop {
        
    //     board.render();

    // }

}
