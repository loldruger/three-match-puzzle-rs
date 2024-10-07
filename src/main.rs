mod piece;
mod board;

use piece::*;
use board::*;

fn main() {
    let mut board = Board::<6, 6>::new();

    loop {
        if !board.is_playable() {
            board.reset();
        }
        let best_matchable = board.get_best_matchable();
        
        board.render();
    }
}
