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
        
    }
}

pub struct Selector {
    x: usize,
    y: usize,
}

impl Selector {
    pub fn new() -> Self {
        Selector { x: 0, y: 0 }
    }

    pub fn move_up(&mut self) {
        self.y = self.y.saturating_sub(1);
    }

    pub fn move_down(&mut self) {
        self.y = self.y.saturating_add(1);
    }

    pub fn move_left(&mut self) {
        self.x = self.x.saturating_sub(1);
    }

    pub fn move_right(&mut self) {
        self.x = self.x.saturating_add(1);
    }

    
}

fn main() {
    let mut board = Board::<6, 6>::new();
    let mut selector = Selector::new();

    board.render();

    println!("Hello, world!");
}
