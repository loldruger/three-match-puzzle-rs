use crate::{Block, Piece};

pub trait Renderable {
    fn render(&self);
}

pub struct Board<const U: usize, const V: usize> {
    pieces: [[Piece; U]; V],
    matchables: Vec<(usize, usize)>,
}

impl<const U: usize, const V: usize> Board<U, V> {
    pub fn new() -> Self {
        Self {
            pieces: Self::reset(),
            matchables: Vec::new(),
        }
    }

    pub fn is_playable(&self) -> bool {
        !self.matchables.is_empty()
    }

    pub fn show_hint(&self) {
        println!("Hint");
    }

    pub fn reset() -> [[Piece; U]; V] {
        let mut pieces = [[Piece::Block(Block::Red); U]; V];
 
        for i in 0..V {
            for j in 0..U {
                pieces[i][j] = rand::random();
            }
        }

        pieces
    }
}

impl<const U: usize, const V: usize> Renderable for Board<U, V> {
    fn render(&self) {
        for i in 0..V {
            for j in 0..U {
                print!("{} ", self.pieces[i][j]);
            }
            println!();
        }
    }
}

