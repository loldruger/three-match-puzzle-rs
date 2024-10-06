use crate::{Block, BlockType};

pub trait Renderable {
    fn render(&self);
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
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
            pieces: Self::reset(),
            matchables: Vec::new(),
            matched: Vec::new(),
            selected: None,
        }
    }

    pub fn is_playable(&self) -> bool {
        !self.matchables.is_empty()
    }

    pub fn get_greatest_matchable(&self) -> Option<&[Position]> {
        self.matchables.iter().max_by_key(|x| x.len()).map(|v| &**v)
    }

    pub fn reset() -> [[BlockType; U]; V] {
        let mut pieces = [[BlockType::Block(Block::Red); U]; V];
 
        for i in 0..V {
            for j in 0..U {
                pieces[i][j] = rand::random();
            }
        }

        pieces
    }

    fn search_matchables(&mut self, pieces: &[[BlockType; U]; V]) {
        // let mut marker = [[false; U]; V];

        self.matchables.clear();
        self.matched.clear();

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

                let x_prev = pieces.get(i - 1).and_then(|a| a.get(j));
                let x_next = pieces.get(i + 1).and_then(|a| a.get(j));
                let y_prev = pieces.get(i).and_then(|a| a.get(j - 1));
                let y_next = pieces.get(i).and_then(|a| a.get(j + 1));

                let mut chunks_h = samples_h.chunk_by(|a, b| a.1 == b.1 + 1);
                let mut chunks_v = samples_v.chunk_by(|a, b| a.0 == b.0 + 1);

                let matched_h = chunks_h.by_ref().filter(|x| x.len() >= 3).collect::<Vec<_>>();
                let matched_v = chunks_v.by_ref().filter(|x| x.len() >= 3).collect::<Vec<_>>();
                
                // [[a, a, a], [a], [a, a], [a, a, a, a], [a]]
                // [[a], [a], [a], [a], [a]]
                // [[a, a], [a, a], [a, a], [a, a], [a, a]]

                let matchables_h = chunks_h 
                    .filter(|x| {


                        x.len() == 1
                    })
                    .collect::<Vec<_>>();

                let matchables_v = chunks_v // [[a, a, a], [a], [a, a], [a, a, a, a], [a]]
                    .filter(|y| {
                        let y_prev = y.get(i - 1).is_some_and(|a| pieces[a.0][a.1] == pieces[i][j]);
                        let y_next = y.get(i + 1).is_some_and(|a| pieces[a.0][a.1] == pieces[i][j]);

                        y.len() == 1
                    })
                    .collect::<Vec<_>>();

                // self.matched.extend(matched_h);
                // self.matched.extend(matched_v);
                // self.matchables.extend(matchables_h);
                // self.matchables.extend(matchables_v);
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

