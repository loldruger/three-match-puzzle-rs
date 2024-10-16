mod block;
mod item_block;

use std::fmt::Display;

pub use block::*;
pub use item_block::*;
use rand::distributions::{Distribution, Standard};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockType {
    Empty,
    Block(Block),
    ItemBlock(ItemBlock),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub const fn shift(&self, dx: isize, dy: isize) -> Self {
        Self {
            x: self.x.wrapping_add_signed(dx),
            y: self.y.wrapping_add_signed(dy),
        }
    }
}

pub struct BlockCell<const U: usize, const V: usize> {
    data: [[BlockType; U]; V],
    matchables: Vec<Vec<Position>>,
    matched: Vec<Vec<Position>>,
}

impl<const U: usize, const V: usize> BlockCell<U, V> {
    pub fn new() -> Self {
        Self {
            data: Self::reset(),
            matchables: Vec::new(),
            matched: Vec::new(),
        }
    }

    pub fn get(&self, pos: Position) -> Option<BlockType> {
        self.data.get(pos.x).and_then(|a| a.get(pos.y)).copied()
    }

    pub fn get_matchables(&self) -> &Vec<Vec<Position>> {
        &self.matchables
    }

    pub fn get_matched(&self) -> &Vec<Vec<Position>> {
        &self.matched
    }

    pub fn reset() -> [[BlockType; U]; V] {
        let mut pieces = [[BlockType::Empty; U]; V];
 
        for i in 0..V {
            for j in 0..U {
                pieces[i][j] = rand::random();
            }
        }

        pieces
    }

    pub fn reset_matchables(&mut self) {
        self.matchables.clear();
    }

    pub fn reset_matched(&mut self) {
        self.matched.clear();
    }

    pub fn swap(&mut self, pos: Position, direction: Direction) -> Result<(), ()> {
        let target = match direction {
            Direction::Up => pos.shift(0, -1),
            Direction::Down => pos.shift(0, 1),
            Direction::Left => pos.shift(-1, 0),
            Direction::Right => pos.shift(1, 0),
        };

        if !(0..U).contains(&target.x) || !(0..V).contains(&target.y) {
            return Err(());
        }

        let temp = self.data[pos.x][pos.y];
        self.data[pos.x][pos.y] = self.data[target.x][target.y];
        self.data[target.x][target.y] = temp;

        Ok(())
    }

    pub fn get_hint_data(&self) -> Option<&Vec<Position>> {
        self.matchables.iter().max_by_key(|x| x.len())
    }

    fn search_matchables(&mut self) {
        self.matchables.clear();

        for i in 0..U*V {
            // let index 
            let a = self.data[i]
                .chunk_by(|a, b| a == b)
                .collect::<Vec<_>>();

            println!("{:?}", a);
        }

        // for j 
        // for j in 0..U {
        //     for i in 0..V {

        //     }
        // }
    }
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