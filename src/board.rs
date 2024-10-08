use std::result;

use crate::{Block, BlockType};

pub trait Renderable {
    fn render(&self);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Position(pub usize, pub usize);

impl Position {
    pub const fn shift(&self, x: isize, y: isize) -> Self {
        Position(self.0.wrapping_add(x as usize), self.1.wrapping_add(y as usize))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Marker(pub bool, pub bool);

pub struct Board<const U: usize, const V: usize> {
    pieces: [[BlockType; U]; V],
    matchables: Vec<Vec<Position>>,
    matched: Vec<Vec<Position>>,
    selected: Option<Position>,
}

impl<const U: usize, const V: usize> Board<U, V> {
    pub fn new() -> Self {
        Self {
            pieces: [[BlockType::Empty; U]; V],
            matchables: Vec::new(),
            matched: Vec::new(),
            selected: None,
        }
    }

    pub fn is_playable(&self) -> bool {
        !self.matchables.is_empty()
    }

    pub fn get_best_matchable(&self) -> Option<&[Position]> {
        self.matchables.iter().max_by_key(|x| x.len()).map(|v| &**v)
    }

    pub fn reset(&mut self) {
        let mut pieces = [[BlockType::Empty; U]; V];
 
        for i in 0..V {
            for j in 0..U {
                pieces[i][j] = rand::random();
            }
        }
        self.pieces = pieces;
        self.search_matchables(&pieces);
    }

    fn inspect(&self, pos: Position) -> Option<BlockType> {
        self.pieces
            .get(pos.0)
            .and_then(|a| a.get(pos.1))
            .copied()
    }

    fn search_matchables(&mut self, pieces: &[[BlockType; U]; V]) {
        self.matchables.clear();
        self.matched.clear();

        self.render();
        let mut iterate_chunks = |chunks: &[&[Position]]| {
            
            for chunk in chunks {
                if chunk.len() >= 3 {
                    self.matched.push(chunk.to_vec());
                    continue;
                }

                let is_dir_h = chunk.first().map(|a| a.0) == chunk.last().map(|a| a.0);

                let item_matchable_prev = chunk.first().and_then(|item| {
                    let (x, y) = if is_dir_h {(0, -2)} else {(-2, 0)};
                    self.inspect(item.shift(x, y)).take_if(|a| self.inspect(*item).is_some_and(|b| *a == b))
                });

                let item_matchable_next = chunk.last().and_then(|item| {
                    let (x, y) = if is_dir_h {(0, 2)} else {(2, 0)};
                    self.inspect(item.shift(x, y)).take_if(|a| self.inspect(*item).is_some_and(|b| *a == b))
                });

                let item_matchable_prev_diagonal = chunk.first().and_then(|item| {
                    let (x, y) = if is_dir_h {(1, -1)} else {(1, -1)};
                    self.inspect(item.shift(x, y)).take_if(|a| self.inspect(*item).is_some_and(|b| *a == b))
                });

                let item_matchable_next_diagonal = chunk.last().and_then(|item| {
                    let (x, y) = if is_dir_h {(1, 1)} else {(1, -1)};
                    self.inspect(item.shift(x, y)).take_if(|a| self.inspect(*item).is_some_and(|b| *a == b))
                });

                if chunk.len() == 2 {
                    if is_item_prev_matched || is_item_next_matched {
                        self.matchables.push(chunk.to_vec());
                    }
                    if is_item_prev_down_matched || is_item_next_down_matched {
                        self.matchables.push(chunk.to_vec());
                    }
                    continue;
                }

                if chunk.len() == 1 {
                    if is_item_prev_down_matched && is_item_next_down_matched {
                        self.matchables.push(chunk.to_vec());
                    }
                }
            }
        };

        let mut marker = [[Marker(false, false); U]; V];

        for i in 0..V {
            for j in 0..U {
                let mut samples_h = Vec::new();
                let mut samples_v = Vec::new();

                for k in 0..U {
                    if !marker[j][k].1 && pieces[j][i] == pieces[j][k] {
                        marker[j][k].1 = true;
                        samples_v.push(Position(j, k));
                    }
                }

                for k in 0..V {
                    if !marker[k][i].0 && pieces[j][i] == pieces[k][i] {
                        marker[k][i].0 = true;
                        samples_h.push(Position(k, i));
                    }
                }
                
                if !samples_h.is_empty() {
                    let chunks_h = samples_h.chunk_by(|a, b| b.0 - a.0 == 1).collect::<Vec<_>>();
                    iterate_chunks(&chunks_h);
                }

                if !samples_v.is_empty() {
                    let chunks_v = samples_v.chunk_by(|a, b| b.1 - a.1 == 1).collect::<Vec<_>>();
                    iterate_chunks(&chunks_v);
                }
            }
        }
    }
}

impl<const U: usize, const V: usize> Renderable for Board<U, V> {
    fn render(&self) {
        for i in 0..V {
            for j in 0..U {
                print!("{} ", self.pieces[j][i]);
            }
            println!();
        }
    }
}

