mod block;
mod item_block;

use std::fmt::Display;

pub use block::*;
pub use item_block::*;
use rand::distributions::{Distribution, Standard};

#[derive(Clone, Copy, PartialEq)]
pub enum BlockType {
    Empty,
    Block(Block),
    ItemBlock(ItemBlock),
}

impl Display for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockType::Empty => write!(f, "  "),
            BlockType::Block(b) => write!(f, "{b}"),
            BlockType::ItemBlock(ib) => write!(f, "{ib}"),
        }
    }
}

impl Distribution<BlockType> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> BlockType {
        BlockType::Block(rng.gen())
    }
}