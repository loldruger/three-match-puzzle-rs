use crate::{Block, BlockType};

pub trait Renderable {
    fn render(&self);
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Position(pub usize, pub usize);

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
        let mut iterate_chunks = |chunks: &Vec<&[Position]>| {
            println!("chunks: {:?}", chunks);
            for chunk in chunks {
                if chunk.len() >= 3 {
                    println!("chunk: {:?}", chunk);
                    self.matched.push(chunk.to_vec());
                    continue;
                }

                let is_item_prev_matched = chunk.first().is_some_and(|a| self.inspect(Position(a.0, a.1.wrapping_sub(2))) == self.inspect(*a));
                let is_item_next_matched = chunk.last().is_some_and(|a| self.inspect(Position(a.0, a.1 + 2)) == self.inspect(*a));
                let is_item_prev_down_matched = chunk.first().is_some_and(|a| self.inspect(Position(a.0.wrapping_sub(1), a.1 + 1)) == self.inspect(*a));
                let is_item_next_down_matched = chunk.last().is_some_and(|a| self.inspect(Position(a.0 + 1, a.1 + 1)) == self.inspect(*a));

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

        for i in 0..V {
            for j in 0..U {
                let mut samples_h = Vec::new();
                let mut samples_v = Vec::new();

                for k in j..U {
                    if pieces[i][j] == pieces[i][k] {
                        samples_h.push(Position(i, k));
                    }
                }

                for k in i..V {
                    if pieces[i][j] == pieces[k][j] {
                        samples_v.push(Position(k, j));
                    }
                }

                let chunks_h = samples_h.split(|a| pieces[a.0][a.1] != pieces[i][j]).collect::<Vec<_>>();
                let chunks_v = samples_v.split(|a| pieces[a.0][a.1] != pieces[i][j]).collect::<Vec<_>>();

                iterate_chunks(&chunks_h);
                iterate_chunks(&chunks_v);
            }
        }
    }
}

impl<const U: usize, const V: usize> Renderable for Board<U, V> {
    fn render(&self) {
        for i in 0..V {
            for j in 0..U {
                print!("{} ", self.pieces[i][j]);
            }
            println!();
        }
    }
}

