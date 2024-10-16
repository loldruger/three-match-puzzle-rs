use std::fmt::Display;
use rand::distributions::{Distribution, Standard};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Block {
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    SkyBlue,
    White
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Red => write!(f, "\x1b[31m■\x1b[0m"),
            Block::Green => write!(f, "\x1b[32m■\x1b[0m"),
            Block::Yellow => write!(f, "\x1b[33m■\x1b[0m"),
            Block::Blue => write!(f, "\x1b[34m■\x1b[0m"),
            Block::Purple => write!(f, "\x1b[35m■\x1b[0m"),
            Block::SkyBlue => write!(f, "\x1b[36m■\x1b[0m"),
            Block::White => write!(f, "\x1b[37m■\x1b[0m")
        }
    }
}

impl Distribution<Block> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Block {
        match rng.gen_range(0..7) {
            0 => Block::Red,
            1 => Block::Green,
            2 => Block::Yellow,
            3 => Block::Blue,
            4 => Block::Purple,
            5 => Block::SkyBlue,
            _ => Block::White,
        }
    }
}