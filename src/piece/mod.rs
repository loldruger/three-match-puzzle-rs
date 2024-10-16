mod block;
mod item_block;

use std::fmt::Display;

use rand::distributions::{Distribution, Standard};

pub use block::*;
pub use item_block::*;

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
    coords_matchable: Vec<Vec<Position>>,
    coords_matched: Vec<Vec<Position>>,
}

impl<const U: usize, const V: usize> BlockCell<U, V> {
    pub fn new() -> Self {
        Self {
            data: Self::reset(),
            coords_matchable: Vec::new(),
            coords_matched: Vec::new(),
        }
    }

    pub fn get(&self, pos: Position) -> Option<BlockType> {
        self.data.get(pos.x).and_then(|a| a.get(pos.y)).copied()
    }

    pub fn get_coords_matchable(&self) -> &Vec<Vec<Position>> {
        &self.coords_matchable
    }

    pub fn get_coords_matched(&self) -> &Vec<Vec<Position>> {
        &self.coords_matched
    }
    
    pub fn get_coords_hint(&self) -> Option<&Vec<Position>> {
        self.coords_matchable.iter().max_by_key(|x| x.len())
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

    pub fn swap(&mut self, coord_from: Position, dir_to: Direction) -> Result<(), ()> {
        let target = match dir_to {
            Direction::Up => coord_from.shift(0, -1),
            Direction::Down => coord_from.shift(0, 1),
            Direction::Left => coord_from.shift(-1, 0),
            Direction::Right => coord_from.shift(1, 0),
        };

        if !(0..U).contains(&target.x) || !(0..V).contains(&target.y) {
            return Err(());
        }

        let temp = self.data[coord_from.x][coord_from.y];
        self.data[coord_from.x][coord_from.y] = self.data[target.x][target.y];
        self.data[target.x][target.y] = temp;

        Ok(())
    }

    pub fn search_coords_matchable(&mut self) {
        self.coords_matchable.clear();

        for i in 0..V*U {
            let a = self.data[i % U]
                .chunk_by(|a, b| a == b)
                .collect::<Vec<_>>();

            println!("{:?}", a);
        }

        for j in 0..U {

        }
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