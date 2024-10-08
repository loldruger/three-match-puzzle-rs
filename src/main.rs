mod piece;
mod board;

use piece::*;
use board::*;

fn main() {
    let mut board = Board::<6, 6>::new();
    board.reset();

    board.is_playable();
    // loop {
        
    //     board.render();

    // }

}
