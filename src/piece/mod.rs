mod block;
mod item_block;

use std::fmt::Display;

pub use block::*;
pub use item_block::*;
use rand::distributions::{Distribution, Standard};

#[derive(Clone, Copy)]
pub enum Piece {
    Block(Block),
    ItemBlock(ItemBlock),
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::Block(b) => write!(f, "{b}"),
            Piece::ItemBlock(ib) => write!(f, "{ib}"),
        }
    }
}

impl Distribution<Piece> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Piece {
        Piece::Block(rng.gen())
    }
}