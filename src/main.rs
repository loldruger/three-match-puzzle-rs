mod piece;
mod board;

use piece::*;
use board::*;

fn main() {
    let mut board = Board::<6, 6>::new();

    board.is_playable();
    board.render();
    // loop {
        
    //     board.render();

    // }

}
