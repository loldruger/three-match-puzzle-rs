use crate::{BlockCell, Position};

pub trait Renderable {
    fn render(&self);
}

pub struct Board<const U: usize, const V: usize> {
    cells: BlockCell<U, V>,
    selected: Option<Position>,
}

impl<const U: usize, const V: usize> Board<U, V> {
    pub fn new() -> Self {
        Self {
            cells: BlockCell::new(),
            selected: None,
        }
    }

    pub fn is_playable(&self) -> bool {
        !self.cells.get_matchables().is_empty()
    }
}

impl<const U: usize, const V: usize> Renderable for Board<U, V> {
    fn render(&self) {
        for i in 0..V {
            for j in 0..U {
                print!("{} ", self.cells.get(Position::new(j, i)).unwrap());
            }
            println!();
        }
    }
}

