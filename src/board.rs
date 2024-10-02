use crate::Block;

pub trait Renderable {
    fn render(&self);
}

pub struct Board<const U: usize, const V: usize> {
    blocks: [[Block; U]; V],
    matchables: Vec<(usize, usize)>,
}

impl<const U: usize, const V: usize> Board<U, V> {
    pub fn new() -> Self {
        Self {
            blocks: Self::reset(),
            matchables: Vec::new(),
        }
    }

    pub fn is_playable(&self) -> bool {
        !self.matchables.is_empty()
    }

    pub fn show_hint(&self) {
        println!("Hint");
    }

    pub fn reset() -> [[Block; U]; V] {
        let mut blocks = [[Block::Red; U]; V];

        for i in 0..V {
            for j in 0..U {
                blocks[i][j] = rand::random();
            }
        }

        blocks
    }
}

impl<const U: usize, const V: usize> Renderable for Board<U, V> {
    fn render(&self) {
        for i in 0..V {
            for j in 0..U {
                print!("{} ", self.blocks[i][j]);
            }
            println!();
        }
    }
}

