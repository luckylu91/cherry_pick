use crate::grid::{Point, Grid};
use std::iter;

#[derive(Clone, Debug)]
pub struct PairScore {
    pub score: i32,
    pub path1: Vec<Point>,
    pub path2: Vec<Point>,
}

impl PairScore {
    pub fn start() -> PairScore {
        PairScore {
            score: 0,
            path1: vec![Point(0, 0)],
            path2: vec![Point(0, 0)],
        }
    }

    pub fn new(score: i32) -> PairScore {
        PairScore {
            score,
            path1: Vec::new(),
            path2: Vec::new(),
        }
    }

    pub fn append(&self, grid: Grid, p1: Point, p2: Point) -> PairScore {
        let delta = p1.score(&grid) + p2.score(&grid);
        let mut pscore = PairScore::new(self.score + delta);
        pscore.path1.extend(self.path1.iter().chain(iter::once(&p1)));
        pscore.path2.extend(self.path2.iter().chain(iter::once(&p2)));
        pscore
    }
}
