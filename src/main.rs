use std::collections::HashMap;
use std::iter;
use std::ops::Deref;

// type Grid = Vec<Vec<i32>>;
#[derive(Debug)]
struct Grid {
    data: Vec<Vec<i32>>,
    size: i32,
}

impl Deref for Grid {
    type Target = Vec<Vec<i32>>;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.data
    }
}

impl Grid {
    fn new(data: Vec<Vec<i32>>) -> Grid {
        if data.len() == 0 {
            panic!("Invalid grid shape: empty grid")
        }
        let height = data.len() as i32;
        let width = data[0].len() as i32;
        if !data.iter().all(|row| row.len() as i32 == width) {
            panic!("Invalid grid shape: not all rows have the same length")
        }
        if width != height {
            panic!("Invalid grid shape: must be a square")
        }
        Grid { data, size: width }
    }

    fn pairs_iter_step_i<'a>(&'a self, i: i32) -> Box<dyn Iterator<Item = PointPair> + 'a> {
        if i == 0  || i >= 2 * self.size - 2 {
            panic!("begin at step 1, stop at step 2 * size - 3")
        }
        let k_max = if i <= self.size - 1 {
            i
        } else {
            2 * self.size - 2 - i
        };
        let k_values = (0..=k_max-1)
            .map(move |k| iter::repeat(k).zip(k+1..=k_max))
            .flat_map(|r| r);

        let coords = k_values.map(move |(k1, k2)| ((k1, k_max - k1), (k2, k_max - k2)));
        let closure: Box<dyn Fn(((i32, i32), (i32, i32))) -> PointPair> =
            if i <= self.size - 1 {
                Box::new(|(p1, p2)| PointPair::from_tuples(p1, p2))
            } else {
                Box::new(|(p1, p2)| PointPair::from_tuples_sym(p1, p2, self.size))
            };
        Box::new(coords.map(closure))
    }

    fn steps_i(&self) ->  Box<dyn Iterator<Item = i32>> {
        Box::new(1 .. 2 * self.size - 2)
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl Point {
    fn from_tuple((x, y): (i32, i32)) -> Self {
        Point(x, y)
    }
    fn from_tuple_sym((x, y): (i32, i32), size: i32) -> Self {
        Point(size - 1 - x, size - 1 - y)
    }
    fn score(&self, grid: &Grid) -> i32 {
        (*grid)[self.0 as usize][self.1 as usize]
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Point(x, y)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct PointPair(Point, Point);

impl PointPair {
    fn from_tuples(x: (i32, i32), y: (i32, i32)) -> Self {
        PointPair(Point::from_tuple(x), Point::from_tuple(y))
    }
    fn from_tuples_sym(x: (i32, i32), y: (i32, i32), size: i32) -> Self {
        PointPair(Point::from_tuple_sym(x, size), Point::from_tuple_sym(y, size))
    }
    fn zero() -> Self {
        PointPair(Point(0, 0), Point(0, 0))
    }
}

struct PairScore {
    score: i32,
    path1: Vec<Point>,
    path2: Vec<Point>,
}

impl PairScore {
    fn start() -> PairScore {
        PairScore {
            score: 0,
            path1: vec![Point(0, 0)],
            path2: vec![Point(0, 0)],
        }
    }

    fn new(score: i32) -> PairScore {
        PairScore {
            score: 0,
            path1: Vec::new(),
            path2: Vec::new(),
        }
    }

    fn append(&self, grid: Grid, p1: Point, p2: Point) -> PairScore {
        let delta = p1.score(&grid) + p2.score(&grid);
        let mut pscore = PairScore::new(self.score + delta);
        pscore.path1.extend(self.path1.iter().chain(iter::once(&p1)));
        pscore.path2.extend(self.path2.iter().chain(iter::once(&p2)));
        pscore
    }
}

type Scores = HashMap<PointPair, PairScore>;

pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    let grid = Grid::new(grid);

    let mut scores = Scores::new();
    scores.insert(PointPair::zero(), PairScore::start());

    for i in grid.steps_i() {
        
    }


    0
}


fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diag_3() {
        let grid = vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ];
        let grid = Grid::new(grid);
        println!("Grid size is 3\n");
        for i in grid.steps_i() {
            println!("i = {}", i);
            grid.pairs_iter_step_i(i).for_each(|x| println!("{:?}", x));
        }
    }

    #[test]
    fn diag_4() {
        let grid = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let grid = Grid::new(grid);
        println!("Grid size is 4\n");
        for i in grid.steps_i() {
            println!("i = {}", i);
            grid.pairs_iter_step_i(i).for_each(|x| println!("{:?}", x));
        }
    }
}
