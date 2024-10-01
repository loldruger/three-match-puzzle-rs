use std::fmt::Display;

#[derive(Clone, Copy)]
pub enum Block {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::A => write!(f, "\x1b[31m■\x1b[0m"),
            Block::B => write!(f, "\x1b[32m■\x1b[0m"),
            Block::C => write!(f, "\x1b[33m■\x1b[0m"),
            Block::D => write!(f, "\x1b[34m■\x1b[0m"),
            Block::E => write!(f, "\x1b[35m■\x1b[0m"),
            Block::F => write!(f, "\x1b[36m■\x1b[0m"),
            Block::G => write!(f, "\x1b[37m■\x1b[0m"),
        }
    }
}

pub trait Renderable {
    fn render(&self);
}

pub struct Board<const U: usize, const V: usize> {
    cells: [[Block; U]; V],
}

impl<const U: usize, const V: usize> Board<U, V> {
    pub fn new() -> Self {
        Board {
            cells: [[Block::A; U]; V],
        }
    }

    pub fn reset(&mut self) {
        self.cells = [[Block::A; U]; V];
    }

}

impl<const U: usize, const V: usize> Renderable for Board<U, V> {
    fn render(&self) {
        for i in 0..V {
            for j in 0..U {
                print!("{}", self.cells[i][j]);
            }
            println!();
        }
    }
}

fn main() {
    let mut board = Board::<6, 6>::new();

    board.render();

}
