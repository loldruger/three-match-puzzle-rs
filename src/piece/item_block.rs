use std::fmt::Display;
use rand::distributions::{Distribution, Standard};

#[derive(Clone, Copy)]
pub enum ItemBlock {
    Vertical,
    Horizontal,
    Bomb,
}

impl Display for ItemBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemBlock::Vertical => write!(f, "\x1b[31mV\x1b[0m"),
            ItemBlock::Horizontal => write!(f, "\x1b[32mH\x1b[0m"),
            ItemBlock::Bomb => write!(f, "\x1b[33mB\x1b[0m"),
        }
    }
}

impl Distribution<ItemBlock> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ItemBlock {
        match rng.gen_range(0..3) {
            0 => ItemBlock::Vertical,
            1 => ItemBlock::Horizontal,
            _ => ItemBlock::Bomb,
        }
    }
}
